pub struct ToolAssembler {
    target_env: crate::TargetEnv,
}

use super::Assembler;

impl Assembler for ToolAssembler {
    fn with_env(target_env: crate::TargetEnv) -> Self {
        Self { target_env }
    }

    fn assemble(
        &self,
        text: &str,
        options: super::AssemblerOptions,
    ) -> Result<crate::binary::Binary, crate::error::Error> {
        let mut cmd = std::process::Command::new("spirv-as");
        cmd.arg("--target-env").arg(self.target_env.to_string());

        if options.preserve_numeric_ids {
            cmd.arg("--preserve-numeric-ids");
        }

        let cmd_output =
            crate::cmd::exec(cmd, Some(text.as_bytes()), crate::cmd::Output::Retrieve)?;

        use std::convert::TryFrom;
        crate::binary::Binary::try_from(cmd_output.binary)
    }

    fn disassemble(
        &self,
        binary: impl AsRef<[u32]>,
        options: super::DisassembleOptions,
    ) -> Result<Option<String>, crate::error::Error> {
        let mut cmd = std::process::Command::new("spirv-dis");

        if options.color {
            cmd.arg("--color");
        }

        if !options.indent {
            cmd.arg("--no-indent");
        }

        if options.show_byte_offset {
            cmd.arg("--offsets");
        }

        if options.no_header {
            cmd.arg("--no-header");
        }

        if !options.use_friendly_names {
            cmd.arg("--raw-id");
        }

        if options.comment {
            cmd.arg("--comment");
        }

        let bytes = crate::binary::from_binary(binary.as_ref());

        let cmd_output = crate::cmd::exec(cmd, Some(bytes), crate::cmd::Output::Retrieve)?;

        String::from_utf8(cmd_output.binary)
            .map_err(|_| crate::error::Error {
                inner: spirv_tools_sys::shared::SpirvResult::InvalidText,
                diagnostic: Some("spirv disassemble returned non-utf8 text".to_owned().into()),
            })
            .map(|s| if s.is_empty() { None } else { Some(s) })
    }
}

impl Default for ToolAssembler {
    fn default() -> Self {
        Self::with_env(crate::TargetEnv::default())
    }
}
