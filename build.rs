use chrono::Utc;
use std::process::Command;

fn main() {
    let rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to execute rustc --version");
    let version_str = String::from_utf8(rustc_version.stdout)
        .expect("rustc output was not valid UTF-8")
        .trim()
        .to_string();
    let build_time = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();

    println!("cargo:rustc-env=RUSTC_VERSION={version_str}");
    println!("cargo:rustc-env=BUILD_TIME={build_time}");
}
