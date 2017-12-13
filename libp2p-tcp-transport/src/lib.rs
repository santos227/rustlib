// Copyright 2017 Parity Technologies (UK) Ltd.
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

// TODO: use this once stable ; for now we just copy-paste the content of the README.md
//#![doc(include = "../README.md")]

//! Implementation of the libp2p `Transport` trait for TCP/IP.
//!
//! Uses [the *tokio* library](https://tokio.rs).
//! 
//! # Usage
//! 
//! Create [a tokio `Core`](https://docs.rs/tokio-core/0.1/tokio_core/reactor/struct.Core.html),
//! then grab a handle by calling the `handle()` method on it, then create a `TcpConfig` and pass
//! the handle.
//!
//! Example:
//!
//! ```
//! extern crate libp2p_tcp_transport;
//! extern crate tokio_core;
//!
//! use libp2p_tcp_transport::TcpConfig;
//! use tokio_core::reactor::Core;
//!
//! # fn main() {
//! let mut core = Core::new().unwrap();
//! let tcp = TcpConfig::new(core.handle());
//! # }
//! ```
//!
//! The `TcpConfig` structs implements the `Transport` trait of the `swarm` library. See the
//! documentation of `swarm` and of libp2p in general to learn how to use the `Transport` trait.

extern crate libp2p_swarm as swarm;
extern crate tokio_core;
extern crate tokio_io;
extern crate multiaddr;
extern crate futures;

use std::io::Error as IoError;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio_core::reactor::Handle;
use tokio_core::net::{TcpStream, TcpListener, TcpStreamNew};
use futures::Future;
use futures::stream::Stream;
use multiaddr::{Multiaddr, Protocol, ToMultiaddr};
use swarm::Transport;

/// Represents the configuration for a TCP/IP transport capability for libp2p.
///
/// Each connection created by this config is tied to a tokio reactor. The TCP sockets created by
/// libp2p will need to be progressed by running the futures and streams obtained by libp2p
/// through the tokio reactor.
#[derive(Debug, Clone)]
pub struct TcpConfig {
    event_loop: Handle,
}

impl TcpConfig {
    /// Creates a new configuration object for TCP/IP. The `Handle` is a tokio reactor the
    /// connections will be created with.
    #[inline]
    pub fn new(handle: Handle) -> TcpConfig {
        TcpConfig { event_loop: handle }
    }
}

impl Transport for TcpConfig {
    /// The raw connection.
    type RawConn = TcpStream;

    /// The listener produces incoming connections.
    type Listener = Box<Stream<Item = (Result<Self::RawConn, IoError>, Multiaddr), Error = IoError>>;

    /// A future which indicates currently dialing to a peer.
    type Dial = TcpStreamNew;

    /// Listen on the given multi-addr.
    /// Returns the address back if it isn't supported.
    fn listen_on(self, addr: Multiaddr) -> Result<(Self::Listener, Multiaddr), (Self, Multiaddr)> {
        if let Ok(socket_addr) = multiaddr_to_socketaddr(&addr) {
            let listener = TcpListener::bind(&socket_addr, &self.event_loop);
            // We need to build the `Multiaddr` to return from this function. If an error happened,
            // just return the original multiaddr.
            let new_addr = match listener {
                Ok(ref l) => if let Ok(new_s_addr) = l.local_addr() {
                    new_s_addr.to_multiaddr().expect("multiaddr generated from socket addr is \
                                                      always valid")
                } else {
                    addr
                }
                Err(_) => addr,
            };
            let future = futures::future::result(listener).map(|listener| {
                    // Pull out a stream of sockets for incoming connections
                    listener.incoming().map(|(sock, addr)| {
                        let addr = addr.to_multiaddr()
                            .expect("generating a multiaddr from a socket addr never fails");
                        (Ok(sock), addr)
                    })
                })
                    .flatten_stream();
            Ok((Box::new(future), new_addr))
        } else {
            Err((self, addr))
        }
    }

    /// Dial to the given multi-addr.
    /// Returns either a future which may resolve to a connection,
    /// or gives back the multiaddress.
    fn dial(self, addr: Multiaddr) -> Result<Self::Dial, (Self, Multiaddr)> {
        if let Ok(socket_addr) = multiaddr_to_socketaddr(&addr) {
            Ok(TcpStream::connect(&socket_addr, &self.event_loop))
        } else {
            Err((self, addr))
        }
    }
}

// This type of logic should probably be moved into the multiaddr package
fn multiaddr_to_socketaddr(addr: &Multiaddr) -> Result<SocketAddr, ()> {
    let protocols = addr.protocol();

    // TODO: This is nonconforming (since a multiaddr could specify TCP first) but we can't fix that
    //       until multiaddrs-rs is improved.
    match (protocols[0], protocols[1]) {
        (Protocol::IP4, Protocol::TCP) => {
            let bs = addr.as_slice();
            Ok(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(bs[1], bs[2], bs[3], bs[4])),
                (bs[6] as u16) << 8 | bs[7] as u16,
            ))
        }
        (Protocol::IP6, Protocol::TCP) => {
            let bs = addr.as_slice();
            if let Ok(Some(s)) = Protocol::IP6.bytes_to_string(&bs[1..17]) {
                if let Ok(ipv6addr) = s.parse() {
                    return Ok(SocketAddr::new(
                        IpAddr::V6(ipv6addr),
                        (bs[18] as u16) << 8 | bs[19] as u16,
                    ));
                }
            }
            Err(())
        }
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::{TcpConfig, multiaddr_to_socketaddr};
    use std;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use tokio_core::reactor::Core;
    use tokio_io;
    use futures::Future;
    use futures::stream::Stream;
    use multiaddr::Multiaddr;
    use swarm::Transport;

    #[test]
    fn multiaddr_to_tcp_conversion() {
        use std::net::Ipv6Addr;

        assert!(multiaddr_to_socketaddr(&Multiaddr::new("/ip4/127.0.0.1/udp/1234").unwrap()).is_err());

        assert_eq!(
            multiaddr_to_socketaddr(&Multiaddr::new("/ip4/127.0.0.1/tcp/12345").unwrap()),
            Ok(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                12345,
            ))
        );
        assert_eq!(
            multiaddr_to_socketaddr(&Multiaddr::new("/ip4/255.255.255.255/tcp/8080").unwrap()),
            Ok(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)),
                8080,
            ))
        );
        assert_eq!(
            multiaddr_to_socketaddr(&Multiaddr::new("/ip6/::1/tcp/12345").unwrap()),
            Ok(SocketAddr::new(
                IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
                12345,
            ))
        );
        assert_eq!(
            multiaddr_to_socketaddr(&Multiaddr::new(
                "/ip6/ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff/tcp/8080",
            ).unwrap()),
            Ok(SocketAddr::new(
                IpAddr::V6(Ipv6Addr::new(
                    65535,
                    65535,
                    65535,
                    65535,
                    65535,
                    65535,
                    65535,
                    65535,
                )),
                8080,
            ))
        );
    }

    #[test]
    fn communicating_between_dialer_and_listener() {
        use std::io::Write;

        std::thread::spawn(move || {
            let mut core = Core::new().unwrap();
            let addr = Multiaddr::new("/ip4/127.0.0.1/tcp/12345").unwrap();
            let tcp = TcpConfig::new(core.handle());
            let handle = core.handle();
            let listener = tcp.listen_on(addr).unwrap().0.for_each(|(sock, _)| {
                // Define what to do with the socket that just connected to us
                // Which in this case is read 3 bytes
                let handle_conn = tokio_io::io::read_exact(sock.unwrap(), [0; 3])
                    .map(|(_, buf)| assert_eq!(buf, [1, 2, 3]))
                    .map_err(|err| panic!("IO error {:?}", err));

                // Spawn the future as a concurrent task
                handle.spawn(handle_conn);

                Ok(())
            });

            core.run(listener).unwrap();
        });
        std::thread::sleep(std::time::Duration::from_millis(100));
        let addr = Multiaddr::new("/ip4/127.0.0.1/tcp/12345").unwrap();
        let mut core = Core::new().unwrap();
        let tcp = TcpConfig::new(core.handle());
        // Obtain a future socket through dialing
        let socket = tcp.dial(addr.clone()).unwrap();
        // Define what to do with the socket once it's obtained
        let action = socket.then(|sock| match sock {
            Ok(mut s) => {
                let written = s.write(&[0x1, 0x2, 0x3]).unwrap();
                Ok(written)
            }
            Err(x) => Err(x),
        });
        // Execute the future in our event loop
        core.run(action).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    #[test]
    fn replace_port_0_in_returned_multiaddr() {
        let core = Core::new().unwrap();
        let tcp = TcpConfig::new(core.handle());

        let addr = Multiaddr::new("/ip4/127.0.0.1/tcp/0").unwrap();
        assert!(addr.to_string().contains("tcp/0"));

        let (_, new_addr) = tcp.listen_on(addr).unwrap();
        assert!(!new_addr.to_string().contains("tcp/0"));
    }
}
