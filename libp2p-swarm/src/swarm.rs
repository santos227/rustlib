// Copyright 2018 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use std::io::Error as IoError;
use futures::{IntoFuture, Future, Stream, Async, Poll};
use futures::sync::mpsc;
use {ConnectionUpgrade, Multiaddr, MuxedTransport, UpgradedNode};

/// Creates a swarm.
///
/// Requires an upgraded transport, and a function or closure that will turn the upgrade into a
/// `Future` that produces a `()`.
///
/// Produces a `SwarmController` and an implementation of `Future`. The controller can be used to
/// control, and the `Future` must be driven to completion in order for things to work.
///
pub fn swarm<T, C, H, F>(upgraded: UpgradedNode<T, C>, handler: H)
                         -> (SwarmController<T, C>, SwarmFuture<T, C, H, F::Future>)
    where T: MuxedTransport + Clone + 'static,      // TODO: 'static :-/
          C: ConnectionUpgrade<T::RawConn> + Clone + 'static,      // TODO: 'static :-/
          H: FnMut(C::Output, Multiaddr) -> F,
          F: IntoFuture<Item = (), Error = IoError>,
{
    let (new_dialers_tx, new_dialers_rx) = mpsc::unbounded();
    let (new_listeners_tx, new_listeners_rx) = mpsc::unbounded();

    let future = SwarmFuture {
        upgraded: upgraded.clone(),
        handler: handler,
        new_listeners: new_listeners_rx,
        next_incoming: upgraded.clone().next_incoming(),
        listeners: Vec::new(),
        listeners_upgrade: Vec::new(),
        dialers: Vec::new(),
        new_dialers: new_dialers_rx,
        to_process: Vec::new(),
    };

    let controller = SwarmController {
        upgraded: upgraded,
        new_listeners: new_listeners_tx,
        new_dialers: new_dialers_tx,
    };

    (controller, future)
}

/// Allows control of what the swarm is doing.
pub struct SwarmController<T, C>
    where T: MuxedTransport + 'static,      // TODO: 'static :-/
          C: ConnectionUpgrade<T::RawConn> + 'static,      // TODO: 'static :-/
{
    upgraded: UpgradedNode<T, C>,
    new_listeners: mpsc::UnboundedSender<Box<Stream<Item = (Box<Future<Item = C::Output, Error = IoError>>, Multiaddr), Error = IoError>>>,
    new_dialers: mpsc::UnboundedSender<(Box<Future<Item = C::Output, Error = IoError>>, Multiaddr)>,
}

impl<T, C> SwarmController<T, C>
    where T: MuxedTransport + Clone + 'static,      // TODO: 'static :-/
          C: ConnectionUpgrade<T::RawConn> + Clone + 'static,      // TODO: 'static :-/
		  C::NamesIter: Clone, // TODO: not elegant
{
    /// Asks the swarm to dial the node with the given multiaddress.
    ///
    /// Once the connection has been open and upgraded, it will be given to the handler.
    // TODO: consider returning a future so that errors can be processed?
    pub fn dial(&self, multiaddr: Multiaddr) -> Result<(), Multiaddr> {
        match self.upgraded.clone().dial(multiaddr.clone()) {
            Ok(dial) => {
                // Ignoring errors if the receiver has been closed, because in that situation
                // nothing is going to be processed anyway.
                let _ = self.new_dialers.unbounded_send((dial, multiaddr));
                Ok(())
            },
            Err((_, multiaddr)) => {
                Err(multiaddr)
            },
        }
    }

    /// Adds a multiaddr to listen on.
    pub fn listen_on(&self, multiaddr: Multiaddr) -> Result<Multiaddr, Multiaddr> {
        match self.upgraded.clone().listen_on(multiaddr) {
            Ok((listener, new_addr)) => {
                // Ignoring errors if the receiver has been closed, because in that situation
                // nothing is going to be processed anyway.
                let _ = self.new_listeners.unbounded_send(listener);
                Ok(new_addr)
            },
            Err((_, multiaddr)) => {
                Err(multiaddr)
            },
        }
    }
}

/// Future that must be driven to completion in order for the swarm to work.
pub struct SwarmFuture<T, C, H, F>
    where T: MuxedTransport + 'static,      // TODO: 'static :-/
          C: ConnectionUpgrade<T::RawConn> + 'static,      // TODO: 'static :-/
{
    upgraded: UpgradedNode<T, C>,
    handler: H,
    new_listeners: mpsc::UnboundedReceiver<Box<Stream<Item = (Box<Future<Item = C::Output, Error = IoError>>, Multiaddr), Error = IoError>>>,
    next_incoming: Box<Future<Item = (C::Output, Multiaddr), Error = IoError>>,
    listeners: Vec<Box<Stream<Item = (Box<Future<Item = C::Output, Error = IoError>>, Multiaddr), Error = IoError>>>,
    listeners_upgrade: Vec<(Box<Future<Item = C::Output, Error = IoError>>, Multiaddr)>,
    dialers: Vec<(Box<Future<Item = C::Output, Error = IoError>>, Multiaddr)>,
    new_dialers: mpsc::UnboundedReceiver<(Box<Future<Item = C::Output, Error = IoError>>, Multiaddr)>,
    to_process: Vec<F>,
}

impl<T, C, H, If, F> Future for SwarmFuture<T, C, H, F>
    where T: MuxedTransport + Clone + 'static,      // TODO: 'static :-/,
          C: ConnectionUpgrade<T::RawConn> + Clone + 'static,      // TODO: 'static :-/
          H: FnMut(C::Output, Multiaddr) -> If,
          If: IntoFuture<Future = F, Item = (), Error = IoError>,
          F: Future<Item = (), Error = IoError>,
{
    type Item = ();
    type Error = IoError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let handler = &mut self.handler;

        match self.next_incoming.poll() {
            Ok(Async::Ready((connec, client_addr))) => {
                self.next_incoming = self.upgraded.clone().next_incoming();
                self.to_process.push(handler(connec, client_addr).into_future());
            },
            Ok(Async::NotReady) => {},
            Err(err) => return Err(err),
        };

        match self.new_listeners.poll() {
            Ok(Async::Ready(Some(new_listener))) => {
                self.listeners.push(new_listener);
            },
            Ok(Async::Ready(None)) | Err(_) => {
                // New listener sender has been closed.
            },
            Ok(Async::NotReady) => {},
        };

        match self.new_dialers.poll() {
            Ok(Async::Ready(Some((new_dialer, multiaddr)))) => {
                self.dialers.push((new_dialer, multiaddr));
            },
            Ok(Async::Ready(None)) | Err(_) => {
                // New dialers sender has been closed.
            },
            Ok(Async::NotReady) => {},
        };

        for n in (0 .. self.listeners.len()).rev() {
            let mut listener = self.listeners.swap_remove(n);
            match listener.poll() {
                Ok(Async::Ready(Some((upgrade, client_addr)))) => {
                    self.listeners.push(listener);
                    self.listeners_upgrade.push((upgrade, client_addr));
                },
                Ok(Async::NotReady) => {
                    self.listeners.push(listener);
                },
                Ok(Async::Ready(None)) => {},
                Err(err) => return Err(err),
            };
        }

        for n in (0 .. self.listeners_upgrade.len()).rev() {
            let (mut upgrade, addr) = self.listeners_upgrade.swap_remove(n);
            match upgrade.poll() {
                Ok(Async::Ready(output)) => {
                    self.to_process.push(handler(output, addr).into_future());
                },
                Ok(Async::NotReady) => {
                    self.listeners_upgrade.push((upgrade, addr));
                },
                Err(err) => return Err(err),
            }
        }

        for n in (0 .. self.dialers.len()).rev() {
            let (mut dialer, addr) = self.dialers.swap_remove(n);
            match dialer.poll() {
                Ok(Async::Ready(output)) => {
                    self.to_process.push(handler(output, addr).into_future());
                },
                Ok(Async::NotReady) => {
                    self.dialers.push((dialer, addr));
                },
                Err(err) => return Err(err),
            }
        }

        for n in (0 .. self.to_process.len()).rev() {
            let mut to_process = self.to_process.swap_remove(n);
            match to_process.poll() {
                Ok(Async::Ready(())) => {},
                Ok(Async::NotReady) => self.to_process.push(to_process),
                Err(err) => return Err(err),
            }
        }

        // TODO: we never return `Ok(Ready)` because there's no way to know whether
        //       `next_incoming()` can produce anything more in the future
        Ok(Async::NotReady)
    }
}
