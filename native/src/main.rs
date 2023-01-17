use anyhow::Result;
use directories::ProjectDirs;
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
use std::{io::Write, time::Duration};

fn post_message(data: &[u8]) -> Result<()> {
    let mut stdout = std::io::stdout().lock();
    stdout.write(&u32::try_from(data.len())?.to_ne_bytes())?;
    stdout.write(data)?;
    stdout.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let project_dirs = ProjectDirs::from("dev", "tombl", "firefox-dyntheme").unwrap();
    let theme_path = project_dirs.config_dir().join("theme.json");
    eprintln!("Watching {:?}", theme_path);

    let data = std::fs::read(&theme_path).unwrap();
    post_message(&data).unwrap();

    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_millis(50), None, tx).unwrap();

    debouncer
        .watcher()
        .watch(&theme_path, RecursiveMode::Recursive)?;

    for _ in rx {
        let data = std::fs::read(&theme_path).unwrap();
        post_message(&data).unwrap();
    }

    Ok(())
}
