use std::{env, fs, path, process::Command};

fn main() {
    let (triple, bazel_root) = {
        let mut args = env::args().skip(1);
        let triple = args.next().expect("expected target triple");
        let bazel_root = args.next().expect("expected bazel root");

        (triple, bazel_root)
    };

    let cwd = "spirv-tools-sys/spirv-tools";

    // Sigh
    {
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
        let mut cmd = Command::new("bazel");
        cmd.args(["build", "--compilation_mode", "opt", "--strip", "always"]);
        cmd.args(BINARIES.iter().map(|b| format!(":{b}")));
        cmd.current_dir(cwd);

        if !cmd.status().expect("bazel not installed").success() {
            panic!("failed to run bazel build");
        }
    }

    let bin_files = {
        let ext = if cfg!(windows) { "exe" } else { "" };

        let out = path::Path::new(&bazel_root).join("bazel-bin");
        let pkg = path::Path::new("tools/bin");

        if pkg.exists() {
            fs::remove_dir_all(&pkg).expect("failed to remove package dir");
        }

        fs::create_dir_all(&pkg).expect("failed to create package dir");

        let mut files = Vec::new();
        for exe in BINARIES {
            let src = out.join(exe).with_extension(ext);
            let tar = pkg.join(exe).with_extension(ext);
            if let Err(err) = fs::copy(&src, &tar) {
                panic!("failed to copy {src:?} to {tar:?}: {err}");
            }
            files.push(tar);
        }

        files
    };

    // Finally, package a zstd compressed tarball
    {
        let mut cmd = Command::new("tar");
        cmd.arg("caf");
        cmd.arg(format!("tools/{triple}.tar.zst"));
        // Strip the leading components so the tarball only contains the files
        cmd.arg("--xform=s,tools/bin/,,");
        cmd.args(bin_files);

        if !cmd.status().expect("tar not installed").success() {
            panic!("failed to package tarball");
        }
    }
}
