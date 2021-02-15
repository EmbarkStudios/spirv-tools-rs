//const TEXT_INPUT: &str = include_str!("test_content.txt");
const ASSEMBLY_INPUT: &str = include_str!("test_content.txt");

use spirv_tools as spv;

use spv::{assembler::Assembler, val::Validator};

#[test]
fn assemblers_match() {
    let cas = spv::assembler::compiled::CompiledAssembler::default();
    let ias = spv::assembler::tool::ToolAssembler::default();

    let cval = spv::val::compiled::CompiledValidator::default();
    let ival = spv::val::tool::ToolValidator::default();

    let cassembled = cas
        .assemble(ASSEMBLY_INPUT, spv::assembler::AssemblerOptions::default())
        .expect("compiled failed to assemble");
    let iassembled = ias
        .assemble(ASSEMBLY_INPUT, spv::assembler::AssemblerOptions::default())
        .expect("tool failed to assemble");

    cval.validate(&cassembled, None)
        .expect("failed to validate input assembly");

    ival.validate(&cassembled, None)
        .expect("failed to validate input assembly");

    cval.validate(&iassembled, None)
        .expect("failed to validate input assembly");

    ival.validate(&iassembled, None)
        .expect("failed to validate input assembly");
}
