//! # `HexView`
//!
//! Easily format a `&[u8]` as hex.
//!
//! ```rust
//! use hex_view::HexView;
//!
//! let ff00_slice: &[u8] = &[255, 0];
//! assert_eq!(format!("{:x}", HexView::from(&ff00_slice)), "ff00")
//!
//! ```
//!
//! `HexView::from` also works on anything that implements `AsRef<[u8]>`, such as a `&Vec<u8>`:
//!
//! ```rust
//! use hex_view::HexView;
//!
//! let ff00_vec: Vec<u8> = vec![255, 0];
//! assert_eq!(format!("{:X}", HexView::from(&ff00_vec)), "FF00")
//! ```

#![no_std]
#![deny(missing_docs)]
#![deny(warnings)]

use core::fmt;
use core::fmt::{Formatter, LowerHex, UpperHex};

/// This struct can be formatted with the `core::fmt::LowerHex` and `core::fmt::UpperHex` formatting
/// traits.
pub struct HexView<'a> {
    bytes: &'a [u8],
}

impl<'a> LowerHex for HexView<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for b in self.bytes {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

impl<'a> UpperHex for HexView<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for b in self.bytes {
            write!(f, "{:02X}", b)?;
        }
        Ok(())
    }
}

impl<'a, T> From<&'a T> for HexView<'a>
where
    T: ?Sized,
    T: AsRef<[u8]>,
    T: 'a,
{
    fn from(bytes: &'a T) -> Self {
        HexView {
            bytes: bytes.as_ref(),
        }
    }
}
