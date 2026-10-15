---
name: new-sketch
description: Start a new experiment at the Hyperprior Institute. Use when hyperprior wants to try an idea, turn a spark into code, or says "let's poke at X", "sketch this", "new experiment".
---

# New sketch

1. Pick a short kebab-case name. Ask hyperprior only if the idea is ambiguous.
   If it grows out of an existing lab (or revives a buried one), use `--from <parent>`.
2. Run `just hp new sketch <name> [--from <parent>]`.
3. Fill in `labs/<name>/lab.toml`: a real `title` and a one-line `summary` phrased
   as the question being poked at.
4. Rewrite `labs/<name>/log/<date>-start.md` with the question, what a satisfying
   answer looks like, and the first thing to try. Keep it to a few lines.
5. If the idea came from `sparks.toml`, leave the spark in place. Sparks are a
   history, not a queue.
6. Get to running code fast: `cargo run -p <name>`. Scrappy is the point.
