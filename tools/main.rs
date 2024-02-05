use std::{env, fs, path, process::Command};

struct Group;

impl Group {
    fn new(group: &str) -> Self {
        println!("::group::{group}");
        Self
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        println!("::endgroup::");
    }
}

fn main() {
    let (triple, bazel_cache) = {
        let mut args = env::args().skip(1);
        let triple = args.next().expect("expected target triple");

        let bc = if env::var_os("CI").is_some() {
            Some(args.next().expect("expected bazel cache directory"))
        } else {
            None
        };

        (triple, bc)
    };

    let cwd = "spirv-tools-sys/spirv-tools";

    // Sigh
    {
        let _s = Group::new("synchronizing additional dependencies");
        let mut cmd = Command::new("python");
        cmd.arg("utils/git-sync-deps");
        cmd.current_dir(cwd);

        if !cmd.status().expect("python not installed").success() {
            panic!("failed to run utils/git-sync-deps");
        }
    }

    const BINARIES: &[&str] = &["spirv-as", "spirv-opt", "spirv-val"];

    // Build the select binaries we/rust-gpu need
    {
        let _s = Group::new("building binaries with Bazel");
        let mut cmd = Command::new("bazel");
        // We use a specific root so that CI can take advantage of the cache,
        // this doesn't change the location of the outputs eg bazel-bin of the
        // workspace
        if let Some(bc) = bazel_cache {
            cmd.arg(format!("--output_user_root={bc}"));
        }

        cmd.args(["build", "--compilation_mode", "opt", "--strip", "always"]);

        cmd.args(BINARIES.iter().map(|b| format!(":{b}")));
        cmd.current_dir(cwd);

        println!("{cmd:#?}");

        if !cmd.status().expect("bazel not installed").success() {
            panic!("failed to run bazel build");
        }
    }

    {
        let _s = Group::new("creating tarball");

        let tar_path = format!("tools/{triple}.tar.zst");

        // Finally, package a zstd compressed tarball
        let tar_file = fs::File::create(&tar_path).expect("failed to create tarball");
        let zstd_stream =
            zstd::stream::write::Encoder::new(tar_file, 3).expect("failed to create zstd encoder");

        let mut tar = tar::Builder::new(zstd_stream);

        // This is the default, but this just makes sure we aren't adding symlinks
        // but rather the files themselves
        tar.follow_symlinks(true);

        let ext = if cfg!(windows) { "exe" } else { "" };
        let out = path::Path::new("spirv-tools-sys/spirv-tools/bazel-bin");

        for exe in BINARIES {
            let src = {
                let mut pb = out.join(exe);
                pb.set_extension(ext);
                pb
            };

            if let Err(err) = tar.append_path_with_name(&src, exe) {
                panic!("failed to append {src:?} to tarball: {err}");
            }

            println!("appended '{exe}'");
        }

        let zstd_stream = tar.into_inner().expect("failed to finish writing tarball");
        let tar_file = zstd_stream.finish().expect("failed to compress tarball");
        tar_file
            .sync_all()
            .expect("failed to flush tarball to disk");

        println!("'{tar_path}' written to disk");
    }
}
