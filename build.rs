use std::io::Error;
use std::process::Command;

fn main() -> Result<(), Error> {
    // Generates the leptodon source file for tailwind.
    // See ./codegen/README.md
    Command::new("cargo")
        .current_dir("codegen")
        .args(["run"])
        .output()
        .expect("failed to execute process");

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=Cargo.toml");
    Ok(())
}
