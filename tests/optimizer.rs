//const TEXT_INPUT: &str = include_str!("test_content.txt");
const ASSEMBLY_INPUT: &[u8] = include_bytes!("assembled_content.spv");

use spirv_tools as spv;

use spv::{assembler::Assembler, opt::Optimizer, val::Validator};
use std::convert::TryFrom;

#[test]
fn compiled_matches_binary() {
    let mut copt = spv::opt::compiled::CompiledOptimizer::default();
    copt.register_size_passes();
    let mut iopt = spv::opt::tool::ToolOptimizer::default();
    iopt.register_size_passes();

    let val = spv::val::create(None);

    let assembled = spv::binary::Binary::try_from(ASSEMBLY_INPUT.to_vec())
        .expect("failed to load assembled output");

    val.validate(&assembled, None)
        .expect("failed to validate input assembly");

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

    let assembler = spv::assembler::create(None);

    let idisasm = assembler
        .disassemble(&iopt_output, spv::assembler::DisassembleOptions::default())
        .unwrap()
        .unwrap();
    let cdisasm = assembler
        .disassemble(&copt_output, spv::assembler::DisassembleOptions::default())
        .unwrap()
        .unwrap();

    if idisasm != cdisasm {
        let diff = similar::TextDiff::from_lines(&idisasm, &cdisasm);
        eprintln!("{}", diff.unified_diff().header("cli", "compiled"));

        panic!("the disassembled text for the cli and the compiled dissassembler did not match");
    }

    val.validate(iopt_output, None)
        .expect("failed to validate tool output");
    val.validate(copt_output, None)
        .expect("failed to validate compiled output");
}
