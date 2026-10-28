#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SbpAddrIpn {
    pub node: u32,
    pub service: u32,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct SbpAddrDtn([libc::c_char; 255]);

#[repr(C)]
#[derive(Clone, Copy)]
pub union SbpAddr {
    pub ipn: SbpAddrIpn,
    pub dtn: SbpAddrDtn,
}

/// Safe Rust view of a Bundle Protocol address.
// Same size as `SockaddrBp`; boxing the DTN variant would lose `Copy`.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Copy)]
pub enum BpAddr {
    Ipn(SbpAddrIpn),
    Dtn(SbpAddrDtn),
}
