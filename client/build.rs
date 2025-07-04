use std::{env, fs, path::PathBuf};
use windows_exe_info as wi;

/// Macro for repeating x amount of times.
macro_rules! repeat {
    ($l:literal, $p:expr) => {
        for _ in 0..$l {
            $p;
        }
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=settings.ini");

    let target_dir = {
        let mut out = env::var("OUT_DIR").map(PathBuf::from)?;
        repeat!(3, out.pop());
        out
    };

    let mut settings_path = target_dir;
    settings_path.push("example_settings.ini");

    fs::copy("settings.ini", settings_path)?;

    #[cfg(windows)]
    {
        wi::manifest("assets/manifest.xml");
        wi::icon::icon_ico("../assets/rpa-watcher.ico");
        wi::versioninfo::VersionInfo::from_cargo_env().link()?;
    }

    Ok(())
}
