#[cfg(not(any(feature = "use-installed-tools", feature = "use-compiled-tools")))]
compile_error!("Enable at least one of `use-compiled-tools` or `use-installed-tools` features");

pub mod assembler;
pub mod binary;
pub mod opt;
pub mod val;

pub mod error;
pub use error::{Error, SpirvResult};

pub use spirv_tools_sys::shared::TargetEnv;

#[cfg(feature = "use-installed-tools")]
pub(crate) mod cmd;
