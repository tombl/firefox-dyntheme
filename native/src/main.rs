use anyhow::Result;
use directories::ProjectDirs;
use signal_hook::{
    consts::{SIGUSR1, SIGUSR2},
    iterator::Signals,
};
use std::{io::Write, path::Path};

fn send_theme(path: &Path) -> Result<()> {
    let data = std::fs::read(path).unwrap();
    let mut stdout = std::io::stdout().lock();
    stdout.write(&u32::try_from(data.len())?.to_ne_bytes())?;
    stdout.write(&data)?;
    stdout.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let project_dirs = ProjectDirs::from("dev", "tombl", "firefox-dyntheme").unwrap();
    let theme_path = project_dirs.config_dir().join("theme.json");
    eprintln!("Reading from {theme_path:?}");

    send_theme(&theme_path).unwrap();

    let mut signals = Signals::new(&[SIGUSR1, SIGUSR2])?;

    for _ in signals.forever() {
        send_theme(&theme_path).unwrap();
    }

    Ok(())
}
