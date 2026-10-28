use crate::{AF_BP, BpAddr, BpScheme, InvalidScheme, SbpAddr};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SockaddrBp {
    sbp_family: libc::sa_family_t,
    /// A bp_scheme_t value (BP_SCHEME_T_IPN or BP_SCHEME_T_DTN).
    sbp_scheme: u32,
    sbp_addr: SbpAddr,
}

impl SockaddrBp {
    pub fn new(addr: BpAddr) -> Self {
        let (scheme, sbp_addr) = match addr {
            BpAddr::Ipn(ipn) => (BpScheme::Ipn, SbpAddr { ipn }),
            BpAddr::Dtn(dtn) => (BpScheme::Dtn, SbpAddr { dtn }),
        };

        Self {
            sbp_family: AF_BP as libc::sa_family_t,
            sbp_scheme: scheme.into(),
            sbp_addr,
        }
    }

    pub fn scheme(&self) -> Result<BpScheme, InvalidScheme> {
        BpScheme::try_from(self.sbp_scheme)
    }

    pub fn addr(&self) -> Result<BpAddr, InvalidScheme> {
        // SAFETY: the scheme selects which union field is active.
        let addr = unsafe {
            match self.scheme()? {
                BpScheme::Ipn => BpAddr::Ipn(self.sbp_addr.ipn),
                BpScheme::Dtn => BpAddr::Dtn(self.sbp_addr.dtn),
            }
        };

        Ok(addr)
    }
}

impl From<BpAddr> for SockaddrBp {
    fn from(addr: BpAddr) -> Self {
        Self::new(addr)
    }
}
