use crate::error;

#[derive(Default)]
pub struct ToolOptimizer {
    target_env: crate::TargetEnv,
    passes: Vec<super::Passes>,
    use_perf_passes: bool,
    use_size_passes: bool,
    //use_vulkan_to_webgpu: bool,
    //use_webgpu_to_vulkan: bool,
    legalize_hlsl: bool,
}

use super::Optimizer;

impl Optimizer for ToolOptimizer {
    fn with_env(target_env: crate::TargetEnv) -> Self {
        Self {
            target_env,
            ..Default::default()
        }
    }

    fn optimize<MC: error::MessageCallback>(
        &self,
        input: impl AsRef<[u32]>,
        msg_callback: &mut MC,
        options: Option<super::Options>,
    ) -> Result<crate::binary::Binary, crate::Error> {
        let mut cmd = std::process::Command::new("spirv-opt");

        // Note here that we don't do cmd.arg("--target-env").arg(self.target_env.to_string());
        // like with the other tools, because opt is "special" and will fail to parse
        // command line options correctly if we don't give join them with the =
        cmd.arg(format!("--target-env={}", self.target_env));

        cmd.args(
            self.passes
                .iter()
                .filter_map(|p| pass_to_string(*p).map(|s| format!("--{}", s))),
        );

        if self.use_perf_passes {
            cmd.arg("-O");
        }

        if self.use_size_passes {
            cmd.arg("-Os");
        }

        if self.legalize_hlsl {
            cmd.arg("--legalize-hlsl");
        }

        if let Some(opts) = options {
            if let Some(max_id_bound) = opts.max_id_bound {
                cmd.arg(format!("--max-id-bound={}", max_id_bound));
            }

            if opts.preserve_bindings {
                cmd.arg("--preserve-bindings");
            }

            if opts.preserve_spec_constants {
                cmd.arg("--preserve-spec-constants");
            }

            if let Some(vopts) = opts.validator_options {
                crate::val::tool::add_options(&mut cmd, vopts);
            }
        }

        let input = crate::binary::from_binary(input.as_ref());

        let cmd_output = crate::cmd::exec(cmd, Some(input), crate::cmd::Output::Retrieve)?;

        for msg in cmd_output.messages {
            msg_callback.on_message(msg);
        }

        crate::binary::Binary::try_from(cmd_output.binary)
    }

    /// Register a single pass with the the optimizer.
    #[inline]
    fn register_pass(&mut self, pass: super::Passes) -> &mut Self {
        self.passes.push(pass);
        self
    }

    /// Registers passes that attempt to improve performance of generated code.
    /// This sequence of passes is subject to constant review and will change
    /// from time to time.
    #[inline]
    fn register_performance_passes(&mut self) -> &mut Self {
        self.use_perf_passes = true;
        self
    }

    /// Registers passes that attempt to improve the size of generated code.
    /// This sequence of passes is subject to constant review and will change
    /// from time to time.
    #[inline]
    fn register_size_passes(&mut self) -> &mut Self {
        self.use_size_passes = true;
        self
    }

    /// Registers passes that attempt to legalize the generated code.
    ///
    /// Note: this recipe is specially designed for legalizing SPIR-V. It should be
    /// used by compilers after translating HLSL source code literally. It should
    /// *not* be used by general workloads for performance or size improvement.
    ///
    /// This sequence of passes is subject to constant review and will change
    /// from time to time.
    #[inline]
    fn register_hlsl_legalization_passes(&mut self) -> &mut Self {
        self.legalize_hlsl = true;
        self
    }
}

fn pass_to_string(pass: super::Passes) -> Option<&'static str> {
    #[allow(clippy::enum_glob_use)]
    use super::Passes::*;

    Some(match pass {
        AggressiveDCE => "eliminate-dead-code-aggressive",
        AmdExtToKhr => "amd-ext-to-khr",
        BlockMerge => "merge-blocks",
        CFGCleanup => "cfg-cleanup",
        CodeSinking => "code-sink",
        CombineAccessChains => "combine-access-chains",
        CompactIds => "compact-ids",
        ConditionalConstantPropagation => "ccp",
        ConvertRelaxedToHalf => "convert-relaxed-to-half",
        CopyPropagateArrays => "copy-propagate-arrays",
        DeadBranchElim => "eliminate-dead-branches",
        DeadInsertElim => "eliminate-dead-inserts",
        DeadVariableElimination => "eliminate-dead-variables",
        DescriptorScalarReplacement => "descriptor-scalar-replacement",
        EliminateDeadConstant => "eliminate-dead-const",
        EliminateDeadFunctions => "eliminate-dead-functions",
        EliminateDeadMembers => "eliminate-dead-members",
        FixStorageClass => "fix-storage-class",
        FlattenDecoration => "flatten-decorations",
        FoldSpecConstantOpAndComposite => "fold-spec-const-op-composite",
        FreezeSpecConstantValue => "freeze-spec-const",
        GraphicsRobustAccess => "graphics-robust-access",
        IfConversion => "if-conversion",
        InlineExhaustive => "inline-entry-points-exhaustive",
        InlineOpaque => "inline-entry-points-opaque",
        InsertExtractElim => "eliminate-insert-extract",
        // This is only part of the --legalize-hlsl meta pass
        InterpolateFixup | Null => return None,
        LocalAccessChainConvert => "convert-local-access-chains",
        LocalMultiStoreElim => "eliminate-local-multi-store",
        LocalRedundancyElimination => "local-redundancy-elimination",
        LocalSingleBlockLoadStoreElim => "eliminate-local-single-block",
        LocalSingleStoreElim => "eliminate-local-single-store",
        LoopInvariantCodeMotion => "loop-invariant-code-motion",
        LoopPeeling => "loop-peeling",
        LoopUnswitch => "loop-unswitch",
        MergeReturn => "merge-return",
        PrivateToLocal => "private-to-local",
        PropagateLineInfo => "propagate-line-info",
        ReduceLoadSize => "reduce-load-size",
        RedundancyElimination => "redundancy-elimination",
        RedundantLineInfoElim => "eliminate-redundant-line-info",
        RemoveUnusedInterfaceVariables => "remove-unused-interface-variables",
        RelaxFloatOps => "relax-float-ops",
        RemoveDuplicates => "remove-duplicates",
        ReplaceInvalidOpcode => "replace-invalid-opcode",
        Simplification => "simplify-instructions",
        SSARewrite => "ssa-rewrite",
        StrengthReduction => "strength-reduction",
        StripDebugInfo => "strip-debug",
        StripNonSemanticInfo => "strip-nonsemantic",
        UnifyConstant => "unify-const",
        UpgradeMemoryModel => "upgrade-memory-model",
        VectorDCE => "vector-dce",
        Workaround1209 => "workaround-1209",
        WrapOpKill => "wrap-opkill",
    })
}
