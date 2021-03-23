#![allow(clippy::unnecessary_wraps)]

const SPIRV_BIN: &[u8] = include_bytes!("wgpu_example_shader.spv");
const SPIRV_OTHER: &[u8] = include_bytes!("invalid_binary.spv");

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
    let cexpected_msg = "invalid cfg:0:0 - Loop header 6[%loop_header] is targeted by 2 back-edge blocks but the standard requires exactly one\n  %loop_header = OpLabel\n";
    let texpected_msg = "internal error:0:0 - Loop header 6[%loop_header] is targeted by 2 back-edge blocks but the standard requires exactly one";
    match (validate_compiled(SPIRV_BIN), validate_tool(SPIRV_BIN)) {
        (Some(resc), Some(rest)) => {
            // assert_eq!(resc, rest);

            assert_eq!(resc.unwrap_err().to_string(), cexpected_msg);
            assert_eq!(rest.unwrap_err().to_string(), texpected_msg);
        }
        (Some(resc), None) => {
            assert_eq!(resc.unwrap_err().to_string(), cexpected_msg);
        }
        (None, Some(rest)) => {
            assert_eq!(rest.unwrap_err().to_string(), texpected_msg);
        }
        _ => {}
    }
}

#[test]
fn verify_error_message() {
    let cexpected_msg = "invalid binary:0:0 - No OpEntryPoint instruction was found. This is only allowed if the Linkage capability is being used.";
    let texpected_msg = "";
    match (validate_compiled(SPIRV_OTHER), validate_tool(SPIRV_OTHER)) {
        (Some(resc), Some(rest)) => {
            // assert_eq!(resc, rest);

            assert_eq!(resc.unwrap_err().to_string(), cexpected_msg);
            assert_eq!(rest.unwrap_err().to_string(), texpected_msg);
        }
        (Some(resc), None) => {
            assert_eq!(resc.unwrap_err().to_string(), cexpected_msg);
        }
        (None, Some(rest)) => {
            assert_eq!(rest.unwrap_err().to_string(), texpected_msg);
        }
        _ => {}
    }
}
