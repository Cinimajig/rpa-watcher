use std::{env, fs, path::PathBuf};
use windows_exe_info as wi;

/// A macro for creating a [`Path`], but for lazy people.
macro_rules! p {
    ($p:expr) => {
        ::std::path::Path::new($p)
    };
    (buf $p:expr) => {
        ::std::path::PathBuf::from($p)
    };
}

/// Macro for repeating x amount of times.
macro_rules! r {
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
        r!(3, out.pop());
        out
    };

    let mut settings_path = target_dir;
    settings_path.push("example_settings.ini");

    fs::copy("settings.ini", settings_path)?;

    #[cfg(windows)]
    {
        wi::manifest(p!("assets/manifest.xml"));
        wi::icon::icon_ico(p!("../assets/rpa-watcher.ico"));
        wi::versioninfo::VersionInfo::from_cargo_env().link()?;
    }

    Ok(())
}
