// The spirv tools use generated code, for now we just replicate the minimum
// generation we need here by calling the *shudders* python script(s) we need
// to in a simple script and commit them to source control, as they only need
// to be regenerated when spirv-headers is updated

use std::{fs, process::Command};

fn python<S: AsRef<std::ffi::OsStr>>(args: impl IntoIterator<Item = S>) -> Result<(), i32> {
    Command::new("python")
        .args(args.into_iter())
        .status()
        .map_err(|_| -1)
        .and_then(|es| {
            if es.success() {
                Ok(())
            } else {
                Err(es.code().unwrap_or(-1))
            }
        })
}

fn main() {
    fs::create_dir_all("generated").expect("unable to create 'generated'");

    python(&[
        "spirv-tools/utils/update_build_version.py",
        "spirv-tools/CHANGES",
        "generated/build-version.inc",
    ])
    .expect("failed to generate build version from spirv-headers");

    enum_string_mapping("unified1");
    core_table("unified1");
    glsl_table("unified1");
    opencl_table("unified1");

    vendor_table("spv-amd-shader-explicit-vertex-parameter", None);
    vendor_table("spv-amd-shader-trinary-minmax", None);
    vendor_table("spv-amd-gcn-shader", None);
    vendor_table("spv-amd-shader-ballot", None);
    vendor_table("debuginfo", None);
    vendor_table("nonsemantic.clspvreflection", None);
    vendor_table("opencl.debuginfo.100", Some("CLDEBUG100_"));

    // This will eventually be moved to spirv-headers
    vendor_table("nonsemantic.shader.debuginfo.100", Some("SHDEBUG100_"));
    generate_header(
        "NonSemanticShaderDebugInfo100",
        "nonsemantic.shader.debuginfo.100",
    );

    registry_table();
}

const HEADERS: &str = "spirv-headers/include/spirv";

fn enum_string_mapping(version: &str) {
    python(&[
        "spirv-tools/utils/generate_grammar_tables.py".to_owned(),
        format!("--spirv-core-grammar={HEADERS}/{version}/spirv.core.grammar.json"),
        format!("--extinst-debuginfo-grammar={HEADERS}/unified1/extinst.debuginfo.grammar.json"),
        format!("--extinst-cldebuginfo100-grammar={HEADERS}/unified1/extinst.opencl.debuginfo.100.grammar.json"),
        "--extension-enum-output=generated/extension_enum.inc".to_owned(),
        "--enum-string-mapping-output=generated/enum_string_mapping.inc".to_owned(),
    ]).expect("failed to generate enum includes from spirv-headers");
}

fn vendor_table(which: &str, prefix: Option<&str>) {
    python(&[
        "spirv-tools/utils/generate_grammar_tables.py".to_owned(),
        format!("--extinst-vendor-grammar={HEADERS}/unified1/extinst.{which}.grammar.json",),
        format!("--vendor-insts-output=generated/{which}.insts.inc"),
        format!(
            "--vendor-operand-kind-prefix={}",
            prefix.unwrap_or_default()
        ),
    ])
    .expect("failed to generate vendor table");
}

// fn vendor_table_local(which: &str, prefix: Option<&str>) {
//     python(&[
//         "spirv-tools/utils/generate_grammar_tables.py".to_owned(),
//         format!(
//             "--extinst-vendor-grammar=spirv-tools/source/extinst.{}.grammar.json",
//             which
//         ),
//         format!("--vendor-insts-output=generated/{}.insts.inc", which),
//         format!(
//             "--vendor-operand-kind-prefix={}",
//             prefix.unwrap_or_default()
//         ),
//     ])
//     .expect("failed to generate vendor table");
// }

fn core_table(which: &str) {
    python(&[
        "spirv-tools/utils/generate_grammar_tables.py".to_owned(),
        format!("--spirv-core-grammar={HEADERS}/unified1/spirv.core.grammar.json"),
        format!("--core-insts-output=generated/core.insts-{which}.inc"),
        format!("--extinst-debuginfo-grammar={HEADERS}/unified1/extinst.debuginfo.grammar.json"),
        format!("--extinst-cldebuginfo100-grammar={HEADERS}/unified1/extinst.opencl.debuginfo.100.grammar.json"),
        format!("--operand-kinds-output=generated/operand.kinds-{which}.inc"),
    ]).expect("failed to generate core table from spirv-headers");
}

fn registry_table() {
    python(&[
        "spirv-tools/utils/generate_registry_tables.py",
        "--xml=spirv-headers/include/spirv/spir-v.xml",
        "--generator=generated/generators.inc",
    ])
    .expect("failed to generate core table from spirv-headers");
}

fn glsl_table(version: &str) {
    python(&[
        "spirv-tools/utils/generate_grammar_tables.py".to_owned(),
        format!("--spirv-core-grammar={HEADERS}/{version}/spirv.core.grammar.json"),
        format!("--extinst-debuginfo-grammar={HEADERS}/unified1/extinst.debuginfo.grammar.json"),
        format!("--extinst-cldebuginfo100-grammar={HEADERS}/unified1/extinst.opencl.debuginfo.100.grammar.json"),
        format!("--extinst-glsl-grammar={HEADERS}/{version}/extinst.glsl.std.450.grammar.json"),
        "--glsl-insts-output=generated/glsl.std.450.insts.inc".to_owned(),
    ]).expect("failed to generate glsl table from spirv-headers");
}

fn opencl_table(version: &str) {
    python(&[
        "spirv-tools/utils/generate_grammar_tables.py".to_owned(),
        format!("--spirv-core-grammar={HEADERS}/{version}/spirv.core.grammar.json"),
        format!("--extinst-debuginfo-grammar={HEADERS}/unified1/extinst.debuginfo.grammar.json"),
        format!("--extinst-cldebuginfo100-grammar={HEADERS}/unified1/extinst.opencl.debuginfo.100.grammar.json"),
        format!("--extinst-opencl-grammar={HEADERS}/{version}/extinst.opencl.std.100.grammar.json"),
        "--opencl-insts-output=generated/opencl.std.insts.inc".to_owned(),
    ]).expect("failed to generate glsl table from spirv-headers");
}

fn generate_header(header_name: &str, grammar: &str) {
    python(&[
        "spirv-tools/utils/generate_language_headers.py".to_owned(),
        format!("--extinst-grammar={HEADERS}/unified1/extinst.{grammar}.grammar.json",),
        format!("--extinst-output-path=generated/{}.h", header_name),
    ])
    .expect("failed to generate C header")
}
