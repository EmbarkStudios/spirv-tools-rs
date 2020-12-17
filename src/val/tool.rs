use std::process::Command;

#[derive(Default)]
pub struct ToolValidator {
    target_env: crate::TargetEnv,
}

use super::Validator;

impl Validator for ToolValidator {
    fn with_env(target_env: crate::TargetEnv) -> Self {
        Self { target_env }
    }

    fn validate(
        &self,
        binary: impl AsRef<[u32]>,
        options: Option<super::ValidatorOptions>,
    ) -> Result<(), crate::error::Error> {
        let mut cmd = Command::new("spirv-val");

        cmd.arg(format!("--target-env={}", self.target_env));

        if let Some(opts) = options {
            // We reuse add options when we run the validator before optimizing,
            // however the optimizer does not recognize limits, so we split them
            // out into a separate function
            add_limits(&mut cmd, &opts.max_limits);
            add_options(&mut cmd, opts);
        }

        let input = crate::binary::from_binary(binary.as_ref());

        crate::cmd::exec(cmd, Some(input), crate::cmd::Output::Ignore)?;
        Ok(())
    }
}

pub(crate) fn add_options(cmd: &mut Command, opts: super::ValidatorOptions) {
    if opts.relax_logical_pointer {
        cmd.arg("--relax-logical-pointer");
    }

    if let Some(true) = opts.relax_block_layout {
        cmd.arg("--relax-block-layout");
    }

    if opts.uniform_buffer_standard_layout {
        cmd.arg("--uniform-buffer-standard-layout");
    }

    if opts.scalar_block_layout {
        cmd.arg("--scalar-block-layout");
    }

    if opts.skip_block_layout {
        cmd.arg("--skip-block-layout");
    }

    if opts.relax_struct_store {
        cmd.arg("--relax-struct-store");
    }

    if opts.before_legalization {
        cmd.arg("--before-hlsl-legalization");
    }
}

fn add_limits(cmd: &mut Command, limits: &[(spirv_tools_sys::val::ValidatorLimits, u32)]) {
    use spirv_tools_sys::val::ValidatorLimits;

    for (limit, val) in limits {
        cmd.arg(format!(
            "--max-{}={}",
            match limit {
                ValidatorLimits::StructMembers => "struct-members",
                ValidatorLimits::StructDepth => "struct-depth",
                ValidatorLimits::LocalVariables => "local-variables",
                ValidatorLimits::GlobalVariables => "global-variables",
                ValidatorLimits::SwitchBranches => "switch-branches",
                ValidatorLimits::FunctionArgs => "function-args",
                ValidatorLimits::ControlFlowNestingDepth => "control-flow-nesting-depth",
                ValidatorLimits::AccessChainIndexes => "access-chain-indexes",
                ValidatorLimits::IdBound => "id-bound",
            },
            val
        ));
    }
}
