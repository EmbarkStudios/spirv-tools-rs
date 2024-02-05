#[cfg(feature = "use-compiled-tools")]
pub mod external {
    use spirv_tools_sys::shared;

    pub struct ExternalBinary {
        inner: *mut shared::Binary,
    }

    impl ExternalBinary {
        #[inline]
        pub(crate) fn new(bin: *mut shared::Binary) -> Self {
            Self { inner: bin }
        }
    }

    impl AsRef<[u32]> for ExternalBinary {
        #[inline]
        fn as_ref(&self) -> &[u32] {
            unsafe { std::slice::from_raw_parts((*self.inner).code, (*self.inner).size) }
        }
    }

    impl AsRef<[u8]> for ExternalBinary {
        #[inline]
        fn as_ref(&self) -> &[u8] {
            unsafe {
                std::slice::from_raw_parts(
                    (*self.inner).code.cast(),
                    (*self.inner).size * std::mem::size_of::<u32>(),
                )
            }
        }
    }

    impl Drop for ExternalBinary {
        #[inline]
        fn drop(&mut self) {
            unsafe {
                shared::binary_destroy(self.inner);
            }
        }
    }
}

pub enum Binary {
    #[cfg(feature = "use-compiled-tools")]
    External(self::external::ExternalBinary),
    OwnedU32(Vec<u32>),
    OwnedU8(Vec<u8>),
}

impl Binary {
    /// Gets a byte array for binary
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }

    /// Gets the words for the binary
    #[inline]
    pub fn as_words(&self) -> &[u32] {
        self.as_ref()
    }
}

impl std::convert::TryFrom<Vec<u8>> for Binary {
    type Error = crate::Error;

    #[inline]
    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        if v.len() % std::mem::size_of::<u32>() != 0 {
            Err(crate::Error {
                inner: spirv_tools_sys::shared::SpirvResult::InvalidBinary,
                diagnostic: None,
            })
        } else {
            Ok(Binary::OwnedU8(v))
        }
    }
}

impl AsRef<[u32]> for Binary {
    #[inline]
    fn as_ref(&self) -> &[u32] {
        match self {
            #[cfg(feature = "use-compiled-tools")]
            Self::External(bin) => bin.as_ref(),
            Self::OwnedU32(v) => v,
            Self::OwnedU8(v) => {
                // If you hit a panic here it's because try_from wasn't used ;)
                to_binary(v).unwrap()
            }
        }
    }
}

impl AsRef<[u8]> for Binary {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        match self {
            #[cfg(feature = "use-compiled-tools")]
            Self::External(bin) => bin.as_ref(),
            Self::OwnedU32(v) => from_binary(v),
            Self::OwnedU8(v) => v,
        }
    }
}

use std::fmt;

impl fmt::Debug for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = match self {
            #[cfg(feature = "use-compiled-tools")]
            Self::External(_) => f.debug_struct("External"),
            Self::OwnedU32(_) => f.debug_struct("OwnedU32"),
            Self::OwnedU8(_) => f.debug_struct("OwnedU8"),
        };

        ds.field("word_count", &self.as_words().len()).finish()
    }
}

/// Transmutes a SPIRV binary, which are stored as 32 bit words, into a more
/// digestible byte array
#[inline]
pub fn from_binary(bin: &[u32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(bin.as_ptr().cast(), std::mem::size_of_val(bin)) }
}

/// Transmutes a regular byte array into a SPIRV binary of 32 bit words. This
/// will fail if the input is not `% sizeof(u32)`
#[inline]
pub fn to_binary(bytes: &[u8]) -> Result<&[u32], crate::Error> {
    if bytes.len() % std::mem::size_of::<u32>() != 0 {
        return Err(crate::Error {
            inner: spirv_tools_sys::shared::SpirvResult::InvalidBinary,
            diagnostic: None,
        });
    }

    #[allow(clippy::size_of_in_element_count)]
    Ok(unsafe {
        std::slice::from_raw_parts(
            bytes.as_ptr().cast(),
            bytes.len() / std::mem::size_of::<u32>(),
        )
    })
}
