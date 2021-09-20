use spirv_tools_sys::{assembler, shared};

pub struct CompiledAssembler {
    inner: *mut shared::ToolContext,
}

use super::Assembler;

impl Assembler for CompiledAssembler {
    fn with_env(target_env: crate::TargetEnv) -> Self {
        Self {
            inner: unsafe { shared::context_create(target_env) },
        }
    }

    fn assemble(
        &self,
        text: &str,
        options: super::AssemblerOptions,
    ) -> Result<crate::binary::Binary, crate::error::Error> {
        unsafe {
            let mut binary = std::ptr::null_mut();
            let mut diagnostic = std::ptr::null_mut();

            let res = assembler::assemble(
                self.inner,
                text.as_ptr().cast(),
                text.len(),
                options.into(),
                &mut binary,
                &mut diagnostic,
            );

            // Always wrap diagnostic, it's fine if it's null
            let diagnostic = crate::error::Diagnostic::from_diag(diagnostic).ok();

            match res {
                shared::SpirvResult::Success => {
                    if binary.is_null() {
                        return Err(crate::error::Error {
                            inner: shared::SpirvResult::InternalError,
                            diagnostic: Some("spirv assemble indicated success but did not return a valid binary".to_owned().into()),
                        });
                    }

                    let bin = crate::binary::external::ExternalBinary::new(binary);
                    Ok(crate::binary::Binary::External(bin))
                }
                other => Err(crate::error::Error {
                    inner: other,
                    diagnostic,
                }),
            }
        }
    }

    fn disassemble(
        &self,
        binary: impl AsRef<[u32]>,
        options: super::DisassembleOptions,
    ) -> Result<Option<String>, crate::error::Error> {
        unsafe {
            let mut text = std::ptr::null_mut();
            let mut diagnostic = std::ptr::null_mut();

            let binary = binary.as_ref();

            let res = assembler::disassemble(
                self.inner,
                binary.as_ptr().cast(),
                binary.len(),
                options.into(),
                &mut text,
                &mut diagnostic,
            );

            // Always wrap diagnostic, it's fine if it's null
            let diagnostic = crate::error::Diagnostic::from_diag(diagnostic).ok();

            match res {
                shared::SpirvResult::Success => {
                    if text.is_null() {
                        return Ok(None);
                    }

                    // Sanity check the text first
                    let disassemble_res = std::str::from_utf8(std::slice::from_raw_parts(
                        (*text).data.cast::<u8>(),
                        (*text).length,
                    ))
                    .map(|disasm| Some(disasm.to_owned()))
                    .map_err(|e| crate::error::Error {
                        inner: shared::SpirvResult::InvalidText,
                        diagnostic: Some(
                            format!("spirv disassemble returned non-utf8 text: {}", e).into(),
                        ),
                    });

                    assembler::text_destroy(text);

                    disassemble_res
                }
                other => Err(crate::error::Error {
                    inner: other,
                    diagnostic,
                }),
            }
        }
    }
}

impl Default for CompiledAssembler {
    fn default() -> Self {
        Self::with_env(crate::TargetEnv::default())
    }
}

impl Drop for CompiledAssembler {
    fn drop(&mut self) {
        unsafe {
            shared::context_destroy(self.inner);
        }
    }
}
