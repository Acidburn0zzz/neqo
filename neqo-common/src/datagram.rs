// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::net::SocketAddr;
use std::ops::Deref;

#[derive(Debug, PartialEq, Clone)]
pub struct Datagram {
    src: SocketAddr,
    dst: SocketAddr,
    d: Vec<u8>,
}

impl Datagram {
    pub fn new<V: Into<Vec<u8>>>(src: SocketAddr, dst: SocketAddr, d: V) -> Datagram {
        Datagram {
            src,
            dst,
            d: d.into(),
        }
    }

    pub fn source(&self) -> SocketAddr {
        self.src
    }

    pub fn destination(&self) -> SocketAddr {
        self.dst
    }
}

impl Deref for Datagram {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.d
    }
}