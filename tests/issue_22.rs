use spirv_tools as spv;
use spv::{assembler::Assembler, opt::Optimizer, val::Validator};

const CONTENT: &str = r#"OpCapability Shader
               OpMemoryModel Logical Simple
               OpEntryPoint Fragment %main "main"
               OpExecutionMode %main OriginUpperLeft
       %file = OpString "file"
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
       %main = OpFunction %void None %3
          %5 = OpLabel
               OpLine %file 1 1
               OpReturn
               OpFunctionEnd"#;

#[test]
fn issue() {
    let cas = spv::assembler::compiled::CompiledAssembler::default();
    let assembled = cas
        .assemble(CONTENT, spv::assembler::AssemblerOptions::default())
        .expect("compiled failed to assemble");

    let val = spv::val::create(None);
    val.validate(&assembled, None)
        .expect("failed to validate input assembly");

    let mut iopt = spv::opt::tool::ToolOptimizer::default();
    iopt.register_pass(spv::opt::Passes::StripDebugInfo);
    let mut copt = spv::opt::compiled::CompiledOptimizer::default();
    copt.register_pass(spv::opt::Passes::StripDebugInfo);

    let iopt_output = iopt
        .optimize(
            &assembled,
            &mut |msg| {
                eprintln!("[tool] optimizer message: {:#?}", msg);
            },
            None,
        )
        .expect("failed to run tool optimizer");

    let copt_output = copt
        .optimize(
            &assembled,
            &mut |msg| {
                eprintln!("[compiled] optimizer message: {:#?}", msg);
            },
            None,
        )
        .expect("failed to run compiled optimizer");

    val.validate(iopt_output, None)
        .expect("failed to validate tool output");
    val.validate(copt_output, None)
        .expect("failed to validate compiled output");
}
