use windows_exe_info as wi;
use std::path::Path;


macro_rules! p {
    ($p:expr) => {
        Path::new($p)
    } 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {    
    wi::manifest(p!("assets/manifest.xml"));
    wi::versioninfo::VersionInfo::from_cargo_env().link()?;

    Ok(())
}