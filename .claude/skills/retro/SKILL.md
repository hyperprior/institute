---
name: retro
description: The institute's self-learning loop. Review the session for corrections, friction and steps repeated by hand, then improve CLAUDE.md, skills and hp. Run it proactively at natural stopping points after substantial work, when the retro-nudge hook asks, or when hyperprior says "retro" or "what did we learn about how we work".
---

# Retro

Look back over the session and make the institute a little better at being a lab.
This is about the *workflow*, not the science. Science goes in log entries
(`write-up`).

## 1. Harvest

Scan the conversation for:
- **Corrections.** hyperprior said "no", "don't", "actually…", or redirected you.
- **Confirmations.** A non-obvious choice hyperprior approved ("yes, exactly").
- **Friction.** Commands that failed, docs that were wrong, permission walls,
  guesses that turned out wrong.
- **Repetition.** Anything done by hand twice or more that a skill or `hp`
  command could do.
- **Gaps.** A question you had to ask that this repo should have answered.

## 2. Route each finding

| Finding | Goes to |
|---|---|
| Personal preference, private context | auto-memory (never the repo) |
| Institute convention or rule | `CLAUDE.md` |
| A procedure | a skill in `.claude/skills/` |
| Something mechanical or repeated | an `hp` command (with a test) |
| A wrong doc or broken step | fix it at the source, now |
| A bigger idea for the machinery | `hp spark "…"` |

Check before writing. Update what's already there rather than adding a duplicate,
and delete rules that turned out wrong.

## 3. Apply or propose

- **Small and clear** (a wording fix, a missing step, a corrected command, a new
  memory): apply it and commit with a `retro:` prefix in the message.
- **Bigger** (a new skill, a new `hp` command, a changed rule, deleting something):
  propose it. Use a lavish plan if it needs a picture, otherwise one short list.
  Wait for a yes.

## 4. Report

End with a few lines: what was learned, what changed, what's proposed. If the
change is notable machinery news, offer a `labs/machinery/log/` entry. If nothing
was worth keeping, say so. An empty retro is fine.
