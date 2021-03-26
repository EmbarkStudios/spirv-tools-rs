#[cfg(feature = "use-compiled-tools")]
pub mod compiled;

#[cfg(feature = "use-installed-tools")]
pub mod tool;

#[derive(Copy, Clone, Default)]
pub struct AssemblerOptions {
    /// Numeric IDs in the binary will have the same values as in the source.
    /// Non-numeric IDs are allocated by filling in the gaps, starting with 1
    /// and going up.
    pub preserve_numeric_ids: bool,
}

#[allow(clippy::from_over_into)]
impl Into<u32> for AssemblerOptions {
    fn into(self) -> u32 {
        // This is weird, the "none" is 1, so I'm not sure if that means having
        // it disables all other options or...?
        let mut res = 0; //assembler::BinaryOptions::None as u32;

        if self.preserve_numeric_ids {
            res |= spirv_tools_sys::assembler::BinaryOptions::PreserveNumberIds as u32;
        }

        res
    }
}

#[derive(Copy, Clone)]
pub struct DisassembleOptions {
    /// Print to stdout.
    pub print: bool,
    /// Add color codes to output
    pub color: bool,
    /// Indent assembly
    pub indent: bool,
    pub show_byte_offset: bool,
    /// Do not output the module header as leading comments in the assembly.
    pub no_header: bool,
    /// Use friendly names where possible.  The heuristic may expand over
    /// time, but will use common names for scalar types, and debug names from
    /// OpName instructions.
    pub use_friendly_names: bool,
    /// Add some comments to the generated assembly
    pub comment: bool,
}

impl Default for DisassembleOptions {
    fn default() -> Self {
        Self {
            print: false,
            color: false,
            indent: true,
            show_byte_offset: false,
            no_header: false,
            use_friendly_names: true,
            comment: true,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<u32> for DisassembleOptions {
    fn into(self) -> u32 {
        let mut res = 0;

        if self.print {
            res |= spirv_tools_sys::assembler::DisassembleOptions::Print as u32;
        }

        if self.color {
            res |= spirv_tools_sys::assembler::DisassembleOptions::Color as u32;
        }

        if self.indent {
            res |= spirv_tools_sys::assembler::DisassembleOptions::Indent as u32;
        }

        if self.show_byte_offset {
            res |= spirv_tools_sys::assembler::DisassembleOptions::ShowByteOffset as u32;
        }

        if self.no_header {
            res |= spirv_tools_sys::assembler::DisassembleOptions::NoHeader as u32;
        }

        if self.use_friendly_names {
            res |= spirv_tools_sys::assembler::DisassembleOptions::FriendlyNames as u32;
        }

        if self.comment {
            res |= spirv_tools_sys::assembler::DisassembleOptions::Comment as u32;
        }

        res
    }
}

pub trait Assembler: Default {
    fn with_env(target_env: crate::TargetEnv) -> Self;
    fn assemble(
        &self,
        text: &str,
        options: AssemblerOptions,
    ) -> Result<crate::binary::Binary, crate::error::Error>;
    fn disassemble(
        &self,
        binary: impl AsRef<[u32]>,
        options: DisassembleOptions,
    ) -> Result<Option<String>, crate::error::Error>;
}

pub fn create(te: Option<crate::TargetEnv>) -> impl Assembler {
    let target_env = te.unwrap_or_default();

    #[cfg(feature = "use-compiled-tools")]
    {
        compiled::CompiledAssembler::with_env(target_env)
    }

    #[cfg(all(feature = "use-installed-tools", not(feature = "use-compiled-tools")))]
    {
        tool::ToolAssembler::with_env(target_env)
    }
}
