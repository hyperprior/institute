//! `lab.toml`: what a project is and where it sits in the lifecycle.

use std::{fmt, fs, path::Path};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

/// Lifecycle stage. Sparks live in `sparks.toml` and have no manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Stage {
    Sketch,
    Lab,
    Library,
    Buried,
}

impl Stage {
    pub fn as_str(self) -> &'static str {
        match self {
            Stage::Sketch => "sketch",
            Stage::Lab => "lab",
            Stage::Library => "library",
            Stage::Buried => "buried",
        }
    }

    /// One step up: sketch → lab → library.
    pub fn promoted(self) -> Result<Stage> {
        match self {
            Stage::Sketch => Ok(Stage::Lab),
            Stage::Lab => Ok(Stage::Library),
            Stage::Library => bail!("already a library; there is no stage above it"),
            Stage::Buried => {
                bail!("buried is terminal; revive it with `hp new sketch <name> --from <buried>`")
            }
        }
    }

    pub fn buried(self) -> Result<Stage> {
        match self {
            Stage::Sketch | Stage::Lab => Ok(Stage::Buried),
            Stage::Library => bail!("libraries aren't buried; archive their repo instead"),
            Stage::Buried => bail!("already buried"),
        }
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub title: String,
    pub stage: Stage,
    pub started: String,
    pub summary: String,
    /// The project this one grew out of.
    #[serde(default)]
    pub parent: Option<String>,
    /// For libraries: where the code lives now.
    #[serde(default)]
    pub repo: Option<String>,
}

impl Manifest {
    pub fn load(path: &Path) -> Result<Manifest> {
        let text =
            fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        toml::from_str(&text).with_context(|| format!("parsing {}", path.display()))
    }
}

/// Rewrite the `stage` line in place so hand-written comments and order survive.
pub fn set_stage(path: &Path, stage: Stage) -> Result<()> {
    let text = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut found = false;
    let lines: Vec<String> = text
        .lines()
        .map(|line| {
            let key = line.split('=').next().unwrap_or("").trim();
            if key == "stage" && !found {
                found = true;
                format!("stage = \"{stage}\"")
            } else {
                line.to_string()
            }
        })
        .collect();
    if !found {
        bail!("no `stage` line in {}", path.display());
    }
    fs::write(path, lines.join("\n") + "\n")?;
    Manifest::load(path).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_transitions() {
        assert_eq!(Stage::Sketch.promoted().unwrap(), Stage::Lab);
        assert_eq!(Stage::Lab.promoted().unwrap(), Stage::Library);
        assert!(Stage::Library.promoted().is_err());
        assert!(Stage::Buried.promoted().is_err());
        assert_eq!(Stage::Sketch.buried().unwrap(), Stage::Buried);
        assert_eq!(Stage::Lab.buried().unwrap(), Stage::Buried);
        assert!(Stage::Library.buried().is_err());
        assert!(Stage::Buried.buried().is_err());
    }

    #[test]
    fn set_stage_preserves_other_lines() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lab.toml");
        let original = "# a comment\ntitle = \"T\"\nstage = \"sketch\"\nstarted = \"2026-10-14\"\nsummary = \"s\"\n";
        fs::write(&path, original).unwrap();
        set_stage(&path, Stage::Lab).unwrap();
        let text = fs::read_to_string(&path).unwrap();
        assert_eq!(text, original.replace("\"sketch\"", "\"lab\""));
        assert_eq!(Manifest::load(&path).unwrap().stage, Stage::Lab);
    }
}
