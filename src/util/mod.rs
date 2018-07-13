// Copyright 2017 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement.  This, along with the Licenses can be
// found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

mod ip_addr;
#[cfg(test)]
pub mod memstream;
mod serde_udp_codec;

pub use self::ip_addr::*;
pub use self::serde_udp_codec::SerdeUdpCodec;

#[cfg(test)]
mod test;

#[cfg(test)]
pub use self::test::*;

/// Tries given expression. Returns boxed future error on failure.
// NOTE: it is duplicate with the one in p2p crate. Consider reusing.
macro_rules! try_bfut {
    ($e:expr) => {
        match $e {
            Ok(t) => t,
            Err(e) => return future::err(e).into_boxed(),
        }
    };
}
