//const TEXT_INPUT: &str = include_str!("test_content.txt");
const ASSEMBLY_INPUT: &[u8] = include_bytes!("assembled_content.spv");

use spirv_tools as spv;

use spv::opt::Optimizer;
use std::convert::TryFrom;

#[test]
fn compiled_matches_binary() {
    let mut copt = spv::opt::compiled::CompiledOptimizer::default();
    copt.register_size_passes();
    let mut iopt = spv::opt::tool::ToolOptimizer::default();
    iopt.register_size_passes();

    let assembled = spv::binary::Binary::try_from(ASSEMBLY_INPUT.to_vec())
        .expect("failed to load assembled output");

    let copt_output = copt
        .optimize(
            &assembled,
            &mut |msg| {
                eprintln!("[compiled] optimizer message: {:#?}", msg);
            },
            None,
        )
        .expect("failed to run compiled optimizer");
    let iopt_output = copt
        .optimize(
            &assembled,
            &mut |msg| {
                eprintln!("[tool] optimizer message: {:#?}", msg);
            },
            None,
        )
        .expect("failedt to run tool optimizer");

    // Due to version differences we don't expect the outputs to be the exact same
    // but the first few bytes should always be the same header, so this is mainly
    // just a test that both of them give _something_
    assert_eq!(
        copt_output.as_bytes().get(0..8),
        iopt_output.as_bytes().get(0..8)
    );
}
