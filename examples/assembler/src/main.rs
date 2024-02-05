use clap::Parser;

/// Create a SPIR-V binary module from SPIR-V assembly text
#[derive(Parser)]
struct Args {
    /// Set the output filename. Use '-' for stdout.
    #[clap(short, default_value = "out.spv")]
    output: String,
    /// Numeric IDs in the binary will have the same values as in the
    /// source. Non-numeric IDs are allocated by filling in the gaps,
    /// starting with 1 and going up.
    #[clap(long = "preserve-numeric-ids")]
    preserve_ids: bool,
    /// Use specified environment.
    #[clap(long = "target-env")]
    target_env: Option<spirv_tools::TargetEnv>,
    /// The input file. Use '-' for stdin.
    #[clap(name = "FILE")]
    input: String,
}

fn main() {
    use spirv_tools::assembler::{self, Assembler};

    let args = Args::parse();

    let contents = if args.input == "-" {
        use std::io::Read;
        let mut v = Vec::with_capacity(1024);
        std::io::stdin()
            .read_to_end(&mut v)
            .expect("failed to read stdin");
        String::from_utf8(v).expect("stdin had invalid utf-8")
    } else {
        std::fs::read_to_string(&args.input).expect("failed to read input file")
    };

    let assembler_opts = assembler::AssemblerOptions {
        preserve_numeric_ids: args.preserve_ids,
    };

    let assembler =
        assembler::compiled::CompiledAssembler::with_env(args.target_env.unwrap_or_default());

    match assembler.assemble(&contents, assembler_opts) {
        Ok(binary) => {
            let len = binary.as_bytes().len();

            if args.output == "-" {
                use std::io::Write;
                std::io::stdout()
                    .lock()
                    .write_all(binary.as_ref())
                    .expect("failed to write binary to stdout");
            } else {
                std::fs::write(&args.output, &binary).expect("failed to write binary");
            }

            println!(
                "wrote {len}b to {}",
                if args.output == "-" {
                    "<stdin>"
                } else {
                    args.output.as_str()
                }
            );
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
