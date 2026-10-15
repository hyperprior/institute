use std::env;

use anyhow::Result;
use clap::{Parser, Subcommand};

use hp::{
    institute::{Institute, find_root},
    manifest::{Manifest, Stage, set_stage},
    scaffold, site, today,
};

/// The Hyperprior Institute's lab machinery.
#[derive(Parser)]
#[command(name = "hp", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Record a spark: a one-line idea, no obligation
    Spark { text: Vec<String> },
    /// Start something new
    New {
        #[command(subcommand)]
        kind: New,
    },
    /// Move a project up a stage: sketch → lab → library
    Promote { name: String },
    /// Bury a sketch or lab, with a log entry to write its epitaph
    Bury { name: String },
    /// Render the site to dist/
    Build {
        /// Include draft log entries (files starting with `_`)
        #[arg(long)]
        drafts: bool,
    },
    /// Preview the site locally, rebuilding on change
    Serve {
        #[arg(long, default_value_t = 8000)]
        port: u16,
    },
}

#[derive(Subcommand)]
enum New {
    /// Scaffold labs/<name>: a runnable crate, a manifest and a first log entry
    Sketch {
        name: String,
        /// The project this grows out of
        #[arg(long)]
        from: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = find_root(&env::current_dir()?)?;
    match cli.command {
        Command::Spark { text } => {
            scaffold::add_spark(&root, &text.join(" "), &today())?;
            println!("sparked");
        }
        Command::New {
            kind: New::Sketch { name, from },
        } => {
            let dir = scaffold::new_sketch(&root, &name, from.as_deref(), &today())?;
            println!("sketch ready: {}  (cargo run -p {name})", dir.display());
        }
        Command::Promote { name } => change_stage(&root, &name, Stage::promoted)?,
        Command::Bury { name } => change_stage(&root, &name, Stage::buried)?,
        Command::Build { drafts } => {
            let inst = Institute::load(&root, drafts)?;
            site::build(&inst, &root.join("dist"))?;
            println!(
                "built {} labs, {} entries → dist/",
                inst.labs.len(),
                inst.entries().len()
            );
        }
        Command::Serve { port } => site::serve::serve(&root, port)?,
    }
    Ok(())
}

fn change_stage(
    root: &std::path::Path,
    name: &str,
    next: fn(Stage) -> Result<Stage>,
) -> Result<()> {
    let dir = root.join("labs").join(name);
    let manifest_path = dir.join("lab.toml");
    let from = Manifest::load(&manifest_path)?.stage;
    let to = next(from)?;
    set_stage(&manifest_path, to)?;
    let body = match to {
        Stage::Buried => {
            format!("# Burying {name}\n\nWhat it was, what we learned, and why we stopped.\n")
        }
        Stage::Library => format!(
            "# {name} becomes a library\n\nWhat got extracted, where it lives now (set `repo` in lab.toml), \
             and who it's for.\n"
        ),
        _ => format!("# {name} becomes a {to}\n\nWhy it stuck, and where it's heading.\n"),
    };
    let log = scaffold::write_log(&dir, &today(), &format!("{to}"), &body)?;
    println!("{name}: {from} → {to}. Write it up: {}", log.display());
    Ok(())
}
