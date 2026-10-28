/// Bundle Protocol endpoint ID scheme.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BpScheme {
    Ipn = 1,
    Dtn = 2,
}

impl TryFrom<u32> for BpScheme {
    type Error = InvalidScheme;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Ipn),
            2 => Ok(Self::Dtn),
            other => Err(InvalidScheme(other)),
        }
    }
}

impl From<BpScheme> for u32 {
    fn from(scheme: BpScheme) -> Self {
        scheme as u32
    }
}

/// Error returned when a raw scheme value is not a known `BpScheme`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("invalid bundle protocol scheme: {0}")]
pub struct InvalidScheme(pub u32);
