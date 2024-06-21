use crate::decode::{Decode, DecodeError};
use crate::encode::{Encode, EncodeError};

/// Variable-width u16 type.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct VarU16(u16);
impl VarU16 {
    /// Creates a new variable length u16.
    /// # Panics
    /// Panics if the value is too large to be encoded as a variable length u16.
    pub fn new(val: u16) -> Self {
        if val > (u16::MAX >> 1) {
            panic!("Value too large for variable length u16");
        }
        Self(val)
    }
    pub fn into_inner(self) -> u16 {
        self.0
    }
    /// Check if the variable length u16 will be wide from the first byte.
    pub(crate) fn check_wide(first: u8) -> bool {
        first > (u8::MAX >> 1) as _
    }
}
impl Encode for VarU16 {
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        if self.0 > (u16::MAX >> 1) {
            return Err(EncodeError::VarShortTooLarge);
        }

        if self.0 > (u8::MAX >> 1) as _ {
            let mut val = self.0.to_le_bytes();
            val[0] |= 1 << 7;
            Ok(val.to_vec())
        } else {
            let val = self.0 as u8;
            Ok(vec![val])
        }
    }
}
impl Decode for VarU16 {
    fn decode(data: impl IntoIterator<Item = u8>) -> Result<Self, DecodeError> {
        let mut data = data.into_iter();
        let first = u8::decode(&mut data)?;
        let wide = first & (1 << 7) != 0;

        if wide {
            let last = u8::decode(&mut data)?;
            let both = [first & u8::MAX >> 1, last];
            Ok(Self(u16::from_be_bytes(both)))
        } else {
            Ok(Self(first as u16))
        }
    }
}
