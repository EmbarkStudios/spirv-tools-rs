use crate::shared;

#[repr(u32)] // SPV_FORCE_32_BIT_ENUM
pub enum BinaryOptions {
    None = 0x1,
    PreserveNumberIds = 1 << 1,
}

#[repr(C)]
pub struct Text {
    pub data: *const std::os::raw::c_char,
    pub length: usize,
}

pub enum DisassembleOptions {
    None = 0x1,
    /// Print to stdout
    Print = 0x2,
    /// Add color codes to output
    Color = 0x4,
    /// Indent assembly
    Indent = 0x8,
    ShowByteOffset = 0x10,
    /// Do not output the module header as leading comments in the assembly.
    NoHeader = 0x20,
    /// Use friendly names where possible.  The heuristic may expand over
    /// time, but will use common names for scalar types, and debug names from
    /// OpName instructions.
    FriendlyNames = 0x40,
    /// Add some comments to the generated assembly
    Comment = 0x80,
}

extern "C" {
    /// Encodes the given SPIR-V assembly text to its binary representation. The
    /// length parameter specifies the number of bytes for text. Encoded binary will
    /// be stored into *binary. Any error will be written into *diagnostic if
    /// diagnostic is non-null, otherwise the context's message consumer will be
    /// used. The generated binary is independent of the context and may outlive it.
    /// The SPIR-V binary version is set to the highest version of SPIR-V supported
    /// by the context's target environment.
    ///
    /// The options parameter is a bit field of
    /// spv_text_to_binary_options_t.
    #[link_name = "spvTextToBinaryWithOptions"]
    pub fn assemble(
        tool: *const shared::ToolContext,
        text: *const std::os::raw::c_char,
        size: usize,
        options: u32,
        binary: *mut *mut shared::Binary,
        diagnostic: *mut *mut crate::diagnostics::Diagnostic,
    ) -> shared::SpirvResult;

    /// Decodes the given SPIR-V binary representation to its assembly text. The
    /// word_count parameter specifies the number of words for binary. The options
    /// parameter is a bit field of spv_binary_to_text_options_t. Decoded text will
    /// be stored into *text. Any error will be written into *diagnostic if
    /// diagnostic is non-null, otherwise the context's message consumer will be
    /// used.
    #[link_name = "spvBinaryToText"]
    pub fn disassemble(
        tool: *const shared::ToolContext,
        binary: *const u32,
        size: usize,
        options: u32,
        out_text: *mut *mut Text,
        diagnostic: *mut *mut crate::diagnostics::Diagnostic,
    ) -> shared::SpirvResult;

    /// Frees an allocated text stream. This is a no-op if the text parameter
    /// is a null pointer.
    #[link_name = "spvTextDestroy"]
    pub fn text_destroy(text: *mut Text);
}
