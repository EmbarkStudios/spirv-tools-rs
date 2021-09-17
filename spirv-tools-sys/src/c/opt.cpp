#include "spirv-tools/optimizer.hpp"
#include <cstring>

struct Optimus;

enum Passes {
    AggressiveDCE,
    AmdExtToKhr,
    BlockMerge,
    CCP,
    CFGCleanup,
    CodeSinking,
    CombineAccessChains,
    CompactIds,
    ConvertRelaxedToHalf,
    CopyPropagateArrays,
    DeadBranchElim,
    DeadInsertElim,
    DeadVariableElimination,
    DescriptorScalarReplacement,
    EliminateDeadConstant,
    EliminateDeadFunctions,
    EliminateDeadMembers,
    FixStorageClass,
    FlattenDecoration,
    FoldSpecConstantOpAndComposite,
    FreezeSpecConstantValue,
    GraphicsRobustAccess,
    IfConversion,
    InlineExhaustive,
    InlineOpaque,
    InsertExtractElim,
    InterpolateFixup,
    LocalAccessChainConvert,
    LocalMultiStoreElim,
    LocalRedundancyElimination,
    LocalSingleBlockLoadStoreElim,
    LocalSingleStoreElim,
    LoopInvariantCodeMotion,
    LoopPeeling,
    LoopUnswitch,
    MergeReturn,
    Null,
    PrivateToLocal,
    PropagateLineInfo,
    ReduceLoadSize,
    RedundancyElimination,
    RedundantLineInfoElim,
    RelaxFloatOps,
    RemoveDuplicates,
    RemoveUnusedInterfaceVariables,
    ReplaceInvalidOpcode,
    Simplification,
    SSARewrite,
    StrengthReduction,
    StripDebugInfo,
    StripReflectInfo,
    UnifyConstant,
    UpgradeMemoryModel,
    VectorDCE,
    Workaround1209,
    WrapOpKill,
};

typedef void (*message_callback)(
    spv_message_level_t level,
    const char* source,
    const spv_position_t* position,
    const char* message,
    void* ctx
);

extern "C" {
    SPIRV_TOOLS_EXPORT Optimus* optimizer_create(spv_target_env target_env) {
        auto* optimizer = new spvtools::Optimizer(target_env);

        return (Optimus*)optimizer;
    }

    SPIRV_TOOLS_EXPORT void optimizer_destroy(Optimus* optimizer) {
        delete (spvtools::Optimizer*)optimizer;
    }

    SPIRV_TOOLS_EXPORT spv_result_t optimizer_run(
        const Optimus* optimizer,
        const uint32_t* input_ptr,
        size_t input_size,
        spv_binary* out_binary,
        message_callback msg_callback,
        void* ctx,
        const spv_optimizer_options options
    ) {
        if (input_ptr == nullptr) {
            return SPV_ERROR_INVALID_POINTER;
        }

        if (out_binary == nullptr) {
            return SPV_ERROR_INVALID_POINTER;
        }

        auto op = (spvtools::Optimizer*)optimizer;

        if (msg_callback) {
            op->SetMessageConsumer([msg_callback, ctx](
                spv_message_level_t level,
                const char* source,
                const spv_position_t& position,
                const char* message) {
                msg_callback(level, source, &position, message, ctx);
            });
        } else {
            // The optimizer keeps the message consumer as state, so if no
            // callback is passed to us, we insert a noop callback to ensure
            // we don't use the state from a previous optimizer run
            op->SetMessageConsumer([](
                spv_message_level_t,
                const char*,
                const spv_position_t&,
                const char*)
                {}
            );
        }

        auto output_buff = std::vector<uint32_t>();
        bool success = false;
        if (options == nullptr) {
            success = op->Run(input_ptr, input_size, &output_buff);
        } else {
            success = op->Run(input_ptr, input_size, &output_buff, options);
        }

        if (!success) {
            return SPV_ERROR_INTERNAL;
        }

        auto word_count = output_buff.size();
        auto data_byte_count = word_count * 4;

        uint32_t* data = new uint32_t[word_count];
        if (data == nullptr) {
            return SPV_ERROR_OUT_OF_MEMORY;
        }

        spv_binary binary = new spv_binary_t { data, word_count };
        if (binary == nullptr) {
            delete[] data;
            return SPV_ERROR_OUT_OF_MEMORY;
        }

        memcpy(data, output_buff.data(), data_byte_count);
        *out_binary = binary;

        return SPV_SUCCESS;
    }

    SPIRV_TOOLS_EXPORT void optimizer_register_pass(Optimus* optimizer, Passes pass) {
        #define PASTEB(a, b) a ## b
        #define PASTEA(a, b) PASTEB(a, b)
        #define PASS(name) \
            case name: \
                op->RegisterPass(spvtools::PASTEA(PASTEA(Create, name), Pass)()); \
                break;

        spvtools::Optimizer* op = (spvtools::Optimizer*)optimizer;

        switch (pass) {
            PASS(AggressiveDCE)
            PASS(AmdExtToKhr)
            PASS(BlockMerge)
            PASS(CCP)
            PASS(CFGCleanup)
            PASS(CodeSinking)
            PASS(CombineAccessChains)
            PASS(CompactIds)
            PASS(ConvertRelaxedToHalf)
            PASS(CopyPropagateArrays)
            PASS(DeadBranchElim)
            PASS(DeadInsertElim)
            PASS(DeadVariableElimination)
            PASS(DescriptorScalarReplacement)
            PASS(EliminateDeadConstant)
            PASS(EliminateDeadFunctions)
            PASS(EliminateDeadMembers)
            PASS(FixStorageClass)
            PASS(FlattenDecoration)
            PASS(FoldSpecConstantOpAndComposite)
            PASS(FreezeSpecConstantValue)
            PASS(GraphicsRobustAccess)
            PASS(IfConversion)
            PASS(InlineExhaustive)
            PASS(InlineOpaque)
            PASS(InsertExtractElim)
            PASS(InterpolateFixup)
            PASS(LocalAccessChainConvert)
            PASS(LocalMultiStoreElim)
            PASS(LocalRedundancyElimination)
            PASS(LocalSingleBlockLoadStoreElim)
            PASS(LocalSingleStoreElim)
            PASS(LoopInvariantCodeMotion)
            PASS(LoopPeeling)
            PASS(LoopUnswitch)
            PASS(MergeReturn)
            PASS(Null)
            PASS(PrivateToLocal)
            PASS(PropagateLineInfo)
            PASS(ReduceLoadSize)
            PASS(RedundancyElimination)
            PASS(RedundantLineInfoElim)
            PASS(RelaxFloatOps)
            PASS(RemoveDuplicates)
            PASS(RemoveUnusedInterfaceVariables)
            PASS(ReplaceInvalidOpcode)
            PASS(Simplification)
            PASS(SSARewrite)
            PASS(StrengthReduction)
            PASS(StripDebugInfo)
            PASS(StripReflectInfo)
            PASS(UnifyConstant)
            PASS(UpgradeMemoryModel)
            PASS(VectorDCE)
            PASS(Workaround1209)
            PASS(WrapOpKill)
        }
    }

    SPIRV_TOOLS_EXPORT void optimizer_register_performance_passes(Optimus* optimizer) {
        ((spvtools::Optimizer*)optimizer)->RegisterPerformancePasses();
    }

    SPIRV_TOOLS_EXPORT void optimizer_register_size_passes(Optimus* optimizer) {
        ((spvtools::Optimizer*)optimizer)->RegisterSizePasses();
    }

    SPIRV_TOOLS_EXPORT void optimizer_register_hlsl_legalization_passes(Optimus* optimizer) {
        ((spvtools::Optimizer*)optimizer)->RegisterLegalizationPasses();
    }
}
