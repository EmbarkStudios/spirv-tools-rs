#[cfg(feature = "use-compiled-tools")]
pub mod external {
    use spirv_tools_sys::shared;

    pub struct ExternalBinary {
        inner: *mut shared::Binary,
    }

    impl ExternalBinary {
        pub(crate) fn new(bin: *mut shared::Binary) -> Self {
            Self { inner: bin }
        }
    }

    impl AsRef<[u32]> for ExternalBinary {
        fn as_ref(&self) -> &[u32] {
            unsafe { std::slice::from_raw_parts((*self.inner).code, (*self.inner).size) }
        }
    }

    impl AsRef<[u8]> for ExternalBinary {
        fn as_ref(&self) -> &[u8] {
            unsafe {
                std::slice::from_raw_parts(
                    (*self.inner).code as *const u8,
                    (*self.inner).size * std::mem::size_of::<u32>(),
                )
            }
        }
    }

    impl Drop for ExternalBinary {
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
    pub fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}

impl std::convert::TryFrom<Vec<u8>> for Binary {
    type Error = crate::Error;

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
    fn as_ref(&self) -> &[u32] {
        match self {
            #[cfg(feature = "use-compiled-tools")]
            Self::External(bin) => bin.as_ref(),
            Self::OwnedU32(v) => &v,
            Self::OwnedU8(v) => {
                // If you hit a panic here it's because try_from wasn't used ;)
                to_binary(&v).unwrap()
            }
        }
    }
}

impl AsRef<[u8]> for Binary {
    fn as_ref(&self) -> &[u8] {
        match self {
            #[cfg(feature = "use-compiled-tools")]
            Self::External(bin) => bin.as_ref(),
            Self::OwnedU32(v) => from_binary(&v),
            Self::OwnedU8(v) => &v,
        }
    }
}

/// Transmutes a SPIRV binary, which are stored as 32 bit words, into a more
/// digestible byte array
pub fn from_binary(bin: &[u32]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            bin.as_ptr() as *const u8,
            bin.len() * std::mem::size_of::<u32>(),
        )
    }
}

/// Transmutes a regular byte array into a SPIRV binary of 32 bit words. This
/// will fail if the input is not `% sizeof(u32)`
pub fn to_binary(bytes: &[u8]) -> Result<&[u32], crate::Error> {
    if bytes.len() % std::mem::size_of::<u32>() != 0 {
        return Err(crate::Error {
            inner: spirv_tools_sys::shared::SpirvResult::InvalidBinary,
            diagnostic: None,
        });
    }

    Ok(unsafe {
        std::slice::from_raw_parts(
            bytes.as_ptr() as *const u32,
            bytes.len() / std::mem::size_of::<u32>(),
        )
    })
}
