
use std::process::Command;
use std::path::Path;

use bootloader_locator::locate_bootloader; 
//クレートが独立してるので、このクレート内ではstdが普通に使える

pub fn main() {
    let bootloader_manifest = locate_bootloader("bootloader").unwrap();

    //相対パスを絶対パスに変換してる（canonicalize）
    let kernel_binary = Path::new("target/x86_64-deepcis_os/debug/deepcis_os").canonicalize().unwrap();
    
    //このクレートのルートディレクトリを取得してる
    let manifest_dir =  Path::new(env!("CARGO_MANIFEST_DIR"));

    let kernel_dir = manifest_dir.parent().unwrap();
    let kernel_manifest =kernel_dir.join("Cargo.toml");
    let target_dir = kernel_dir.join("target");
    let out_dir = kernel_binary.parent().unwrap();

    //we ensure that we use the exact same cargo version for compiling 
    //the bootloader crate, which is important when using non-standard cargo versions.
    let mut build_cmd = Command::new(env!("CARGO"));

    build_cmd.arg("builder");
    build_cmd.arg("--kernel-manifest").arg(&kernel_manifest);
    build_cmd.arg("--kernel-binary").arg(&kernel_binary);
    build_cmd.arg("--target-dir").arg(&target_dir);
    build_cmd.arg("--out-dir").arg(&out_dir);

    let bootloader_dir = bootloader_manifest.parent().unwrap();
    build_cmd.current_dir(bootloader_dir);

    //run command
    let exit_status = build_cmd.status().unwrap();
    if !exit_status.success() {
        panic!("build failed");
    }
}