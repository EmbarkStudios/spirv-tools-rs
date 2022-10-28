#![allow(clippy::unnecessary_wraps)]

const SPIRV_BIN: &[u8] = include_bytes!("wgpu_example_shader.spv");

fn validate_compiled(_input: &[u8]) -> Option<Result<(), spirv_tools::Error>> {
    #[cfg(feature = "use-compiled-tools")]
    {
        use spirv_tools::val::{compiled::CompiledValidator, Validator};
        let cv = CompiledValidator::default();
        Some(cv.validate(spirv_tools::binary::to_binary(_input).unwrap(), None))
    }
    #[cfg(not(feature = "use-compiled-tools"))]
    None
}

fn validate_tool(_input: &[u8]) -> Option<Result<(), spirv_tools::Error>> {
    #[cfg(feature = "use-installed-tools")]
    {
        use spirv_tools::val::{tool::ToolValidator, Validator};
        let cv = ToolValidator::default();
        Some(cv.validate(spirv_tools::binary::to_binary(_input).unwrap(), None))
    }
    #[cfg(not(feature = "use-installed-tools"))]
    None
}

#[test]
fn gets_error_message() {
    let expected_msg = "error:0:0 - Loop header '6[%loop_header]' is targeted by 2 back-edge blocks but the standard requires exactly one";
    let expected_notes = "  %loop_header = OpLabel";

    match (validate_compiled(SPIRV_BIN), validate_tool(SPIRV_BIN)) {
        (Some(resc), Some(rest)) => {
            let cstr = resc.unwrap_err().to_string();
            let tstr = rest.unwrap_err().to_string();

            assert_eq!(cstr, tstr);

            assert_eq!(&tstr[..113], expected_msg);
            assert_eq!(&cstr[..113], expected_msg);

            assert_eq!(&cstr[113 + 1..], expected_notes);
            assert_eq!(&tstr[113 + 1..], expected_notes);
        }
        (Some(resc), None) => {
            let diag = resc.unwrap_err().diagnostic.unwrap();
            assert_eq!(diag.line, 0);
            assert_eq!(diag.column, 0);
            assert_eq!(diag.message, &expected_msg[12..]);
            assert_eq!(diag.notes, expected_notes);
        }
        (None, Some(rest)) => {
            let diag = rest.unwrap_err().diagnostic.unwrap();
            assert_eq!(diag.line, 0);
            assert_eq!(diag.column, 0);
            assert_eq!(diag.message, &expected_msg[12..]);
            assert_eq!(diag.notes, expected_notes);
        }
        _ => {}
    }
}
