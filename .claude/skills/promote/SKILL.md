---
name: promote
description: Move a Hyperprior Institute project up a stage (sketch → lab → library). Use when hyperprior says a sketch has stuck, or code is ready to leave home as a library.
---

# Promote

- **sketch → lab**: only when hyperprior says it stuck. Run `just hp promote <name>`
  and fill in the generated log entry: why it stuck and where it's heading.
- **lab → library**: the code is leaving home. Before promoting:
  1. Confirm the code to extract is used in real places (see `propose-extraction`).
  2. Ask before creating a repo in the `hyperprior` GitHub org.
  3. After hyperprior approves, create the repo, move the code, and set `repo = "<url>"`
     in `lab.toml`. Keep `labs/<name>/` as a stub: `lab.toml` and `log/`. Add it to
     `exclude` in the root `Cargo.toml` once it no longer has a crate.
  4. Run `just hp promote <name>` and write the entry.

Never promote on speculation. Promotion happens by extraction, not by planning.
