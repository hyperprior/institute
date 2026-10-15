//! Creating things: sparks, sketches, and log entries.

use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};

const SKETCH_CARGO: &str = include_str!("../templates/sketch/Cargo.toml.tmpl");
const SKETCH_MAIN: &str = include_str!("../templates/sketch/main.rs.tmpl");
const SKETCH_LAB: &str = include_str!("../templates/sketch/lab.toml.tmpl");
const SKETCH_START: &str = include_str!("../templates/sketch/start.md.tmpl");

pub fn add_spark(root: &Path, text: &str, date: &str) -> Result<()> {
    let text = text.trim();
    if text.is_empty() {
        bail!("a spark needs some text");
    }
    let quoted = toml::Value::String(text.to_string()).to_string();
    let mut file = OpenOptions::new()
        .append(true)
        .open(root.join("sparks.toml"))?;
    write!(file, "\n[[spark]]\ndate = \"{date}\"\ntext = {quoted}\n")?;
    Ok(())
}

/// Scaffold `labs/<name>`: a runnable crate, a manifest and a first log entry.
pub fn new_sketch(root: &Path, name: &str, parent: Option<&str>, date: &str) -> Result<PathBuf> {
    let valid = name.starts_with(|c: char| c.is_ascii_lowercase())
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
    if !valid {
        bail!("sketch names are lowercase letters, digits and dashes, starting with a letter");
    }
    let dir = root.join("labs").join(name);
    if dir.exists() {
        bail!("labs/{name} already exists");
    }
    if let Some(parent) = parent
        && !root.join("labs").join(parent).join("lab.toml").is_file()
    {
        bail!("no lab named `{parent}` to grow from");
    }

    let parent_line = parent
        .map(|p| format!("parent = \"{p}\"\n"))
        .unwrap_or_default();
    let fill = |tmpl: &str| {
        tmpl.replace("{{name}}", name)
            .replace("{{date}}", date)
            .replace("{{parent}}\n", &parent_line)
    };

    fs::create_dir_all(dir.join("src"))?;
    fs::write(dir.join("Cargo.toml"), fill(SKETCH_CARGO))?;
    fs::write(dir.join("src/main.rs"), fill(SKETCH_MAIN))?;
    fs::write(dir.join("lab.toml"), fill(SKETCH_LAB))?;
    write_log(&dir, date, "start", &fill(SKETCH_START))?;
    Ok(dir)
}

/// Write `log/<date>-<slug>.md`, adding a numeric suffix rather than overwriting.
pub fn write_log(lab_dir: &Path, date: &str, slug: &str, body: &str) -> Result<PathBuf> {
    let log = lab_dir.join("log");
    fs::create_dir_all(&log)?;
    let mut path = log.join(format!("{date}-{slug}.md"));
    let mut n = 2;
    while path.exists() {
        path = log.join(format!("{date}-{slug}-{n}.md"));
        n += 1;
    }
    fs::write(&path, body).with_context(|| format!("writing {}", path.display()))?;
    Ok(path)
}
