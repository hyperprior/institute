---
name: write-up
description: Draft a log entry for a Hyperprior Institute lab from recent work. Use when hyperprior says "write this up", "log this", "what did we learn", or at the end of a working session on a lab.
---

# Write-up

1. Find what changed in the lab: `git diff`/`git log -- labs/<name>`, outputs,
   figures, and the conversation so far.
2. Draft `labs/<name>/log/_<YYYY-MM-DD>-<slug>.md`. The leading `_` makes it a
   draft that shows only under `hp serve`.
   - The first line is `# Title`. No frontmatter.
   - Say what was tried, what happened, and what's next. Include negative results.
   - Math is `$…$` / `$$…$$`. Figures go in `labs/<name>/figures/` and are linked
     as `../figures/<file>.svg`.
   - Write plainly in the institute's voice. Refer to the human only as hyperprior.
3. Tell hyperprior where the draft is. They publish it by dropping the `_`.
