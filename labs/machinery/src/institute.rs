//! The whole institute, loaded from the repo: labs, their logs, and sparks.

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::manifest::{Manifest, Stage};

pub struct Institute {
    pub root: PathBuf,
    pub about: String,
    pub labs: Vec<Lab>,
    pub sparks: Vec<Spark>,
}

pub struct Lab {
    pub name: String,
    pub dir: PathBuf,
    pub manifest: Manifest,
    /// Newest first.
    pub entries: Vec<Entry>,
}

/// A log entry: `log/YYYY-MM-DD-slug.md`, or `log/_…` for a draft.
pub struct Entry {
    /// The file stem, e.g. `2026-10-14-founding`; also the URL segment.
    pub slug: String,
    pub date: String,
    pub title: String,
    /// Markdown without the leading `# title` line.
    pub body: String,
    pub draft: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Spark {
    pub date: String,
    pub text: String,
}

#[derive(Deserialize)]
struct SparkFile {
    #[serde(default)]
    spark: Vec<Spark>,
}

/// The institute root is the nearest ancestor holding `sparks.toml`.
pub fn find_root(start: &Path) -> Result<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join("sparks.toml").is_file())
        .map(Path::to_path_buf)
        .context("not inside the institute (no sparks.toml in any parent directory)")
}

impl Institute {
    pub fn load(root: &Path, drafts: bool) -> Result<Institute> {
        let about = fs::read_to_string(root.join("about.md")).unwrap_or_default();
        let sparks_text = fs::read_to_string(root.join("sparks.toml"))?;
        let mut sparks = toml::from_str::<SparkFile>(&sparks_text)
            .context("parsing sparks.toml")?
            .spark;
        sparks.sort_by(|a, b| b.date.cmp(&a.date));

        let mut labs = Vec::new();
        let labs_dir = root.join("labs");
        if labs_dir.is_dir() {
            for dir in sorted_dirs(&labs_dir)? {
                let manifest_path = dir.join("lab.toml");
                if !manifest_path.is_file() {
                    bail!("{} has no lab.toml", dir.display());
                }
                let name = dir.file_name().unwrap().to_string_lossy().into_owned();
                let manifest = Manifest::load(&manifest_path)?;
                let entries = load_entries(&dir.join("log"), drafts)?;
                labs.push(Lab {
                    name,
                    dir,
                    manifest,
                    entries,
                });
            }
        }

        let inst = Institute {
            root: root.to_path_buf(),
            about,
            labs,
            sparks,
        };
        inst.check_lineage()?;
        Ok(inst)
    }

    pub fn lab(&self, name: &str) -> Option<&Lab> {
        self.labs.iter().find(|lab| lab.name == name)
    }

    pub fn children<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Lab> {
        self.labs
            .iter()
            .filter(move |lab| lab.manifest.parent.as_deref() == Some(name))
    }

    /// Every entry across every lab, newest first.
    pub fn entries(&self) -> Vec<(&Lab, &Entry)> {
        let mut all: Vec<_> = self
            .labs
            .iter()
            .flat_map(|lab| lab.entries.iter().map(move |e| (lab, e)))
            .collect();
        all.sort_by(|a, b| (&b.1.date, &b.1.slug).cmp(&(&a.1.date, &a.1.slug)));
        all
    }

    pub fn labs_in(&self, stage: Stage) -> impl Iterator<Item = &Lab> {
        self.labs
            .iter()
            .filter(move |lab| lab.manifest.stage == stage)
    }

    /// Parents must exist and lineage must not loop.
    fn check_lineage(&self) -> Result<()> {
        let parents: HashMap<&str, Option<&str>> = self
            .labs
            .iter()
            .map(|lab| (lab.name.as_str(), lab.manifest.parent.as_deref()))
            .collect();
        for lab in &self.labs {
            let mut seen = vec![lab.name.as_str()];
            let mut cursor = lab.manifest.parent.as_deref();
            while let Some(parent) = cursor {
                let Some(next) = parents.get(parent) else {
                    bail!(
                        "lab `{}` names parent `{parent}`, which doesn't exist",
                        lab.name
                    );
                };
                if seen.contains(&parent) {
                    bail!("lineage loop through `{parent}`");
                }
                seen.push(parent);
                cursor = *next;
            }
        }
        Ok(())
    }
}

fn sorted_dirs(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut dirs: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    Ok(dirs)
}

fn load_entries(log_dir: &Path, drafts: bool) -> Result<Vec<Entry>> {
    let mut entries = Vec::new();
    if !log_dir.is_dir() {
        return Ok(entries);
    }
    for item in fs::read_dir(log_dir)? {
        let path = item?.path();
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        let stem = path.file_stem().unwrap().to_string_lossy().into_owned();
        let draft = stem.starts_with('_');
        if draft && !drafts {
            continue;
        }
        let slug = stem.trim_start_matches('_').to_string();
        let date = slug.get(..10).unwrap_or_default();
        if date.parse::<jiff::civil::Date>().is_err() {
            bail!("{}: log files are named YYYY-MM-DD-slug.md", path.display());
        }
        let text = fs::read_to_string(&path)?;
        let (title, body) = split_title(&text);
        entries.push(Entry {
            date: date.to_string(),
            title: title.unwrap_or_else(|| slug[10..].trim_matches('-').replace('-', " ")),
            slug,
            body,
            draft,
        });
    }
    entries.sort_by(|a, b| b.slug.cmp(&a.slug));
    Ok(entries)
}

/// Take a leading `# Title` line off a markdown document.
pub fn split_title(md: &str) -> (Option<String>, String) {
    let trimmed = md.trim_start();
    if let Some(rest) = trimmed.strip_prefix("# ") {
        let (title, body) = rest.split_once('\n').unwrap_or((rest, ""));
        (Some(title.trim().to_string()), body.to_string())
    } else {
        (None, md.to_string())
    }
}
