mod addr;
mod scheme;
mod sockaddr;

pub use addr::{BpAddr, SbpAddr, SbpAddrDtn, SbpAddrIpn};
pub use scheme::{BpScheme, InvalidScheme};
pub use sockaddr::SockaddrBp;

/// Address family number for Bundle Protocol. Randomly chosen.
pub const AF_BP: libc::c_int = 60518;
