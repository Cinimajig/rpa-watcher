use windows_exe_info as wi;
use std::{path::{Path, PathBuf}, fs, env};

macro_rules! p {
    ($p:expr) => {
        Path::new($p)
    } 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=settings.ini");
    
    let target_dir = {
        let mut out = env::var("OUT_DIR").map(PathBuf::from)?;
        out.pop();
        out.pop();
        out.pop();

        out
    };

    let mut settings_path = target_dir.clone();
    settings_path.push("example_settings.ini");

    fs::copy(
        "settings.ini",
        settings_path
    )?;

    wi::manifest(p!("assets/manifest.xml"));
    wi::versioninfo::VersionInfo::from_cargo_env().link()?;

    Ok(())
}
