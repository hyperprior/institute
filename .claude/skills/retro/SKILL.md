---
name: retro
description: The institute's self-learning loop. Review the session for corrections, friction and steps repeated by hand, then improve CLAUDE.md, skills and hp. Run it proactively at natural stopping points after substantial work, when the retro-nudge hook asks, or when hyperprior says "retro" or "what did we learn about how we work".
---

# Retro

(If the Skill tool doesn't know `retro` yet, because skills added mid-session load
on the next session, read this file and follow it directly.)

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

## 2. Hunt context killers

Every retro, not just when asked. Context is a cost, like wall-clock and memory.
Look at what this session actually loaded and spent:
- **Always-on load.** Hook output (SessionStart, UserPromptSubmit), the skill
  listing, MCP tools loaded in full rather than deferred, the agent list, and the
  size of CLAUDE.md and the memory index. Anything not used here is a suspect.
- **In-session waste.** Big tool outputs that got read but not used, whole files
  read where a range would do, and repeated reads of the same thing.
- **Numbers.** Call `mcp__ccd_session_mgmt__get_usage` (session `self`). It
  returns tokens per category (MCP tools, system tools, skills, memory, agents).
  Compare them with the last retro's numbers in the `context-trim` memory. It
  merges loaded and deferred tools, so use it for trends, not to match `/context`.

Present a short plan: each killer, its rough size, the cut, and its scope
(repo, project-local `settings.local.json`, or global). Project-local and repo
cuts follow the rules in step 4. Global cuts always wait for a yes, because
`~/.claude` is shared with work. After a cut lands, record the new numbers in the
memory.

## 3. Route each finding

| Finding | Goes to |
|---|---|
| Personal preference, private context | auto-memory (never the repo) |
| Institute convention or rule | `CLAUDE.md` |
| A procedure | a skill in `.claude/skills/` |
| Something mechanical or repeated | an `hp` command (with a test) |
| A wrong doc or broken step | fix it at the source, now |
| A bigger idea for the machinery | mention it in the report; spark it only if hyperprior says so |

Check before writing. Update what's already there rather than adding a duplicate,
and delete rules that turned out wrong.

## 4. Apply or propose

- **Small and clear** (a wording fix, a missing step, a corrected command, a new
  memory): apply it and commit with a `retro:` prefix in the message.
- **Bigger** (a new skill, a new `hp` command, a changed rule, deleting something):
  propose it. Use a lavish plan if it needs a picture, otherwise one short list.
  Wait for a yes.

## 5. Report

End with a few lines: what was learned, what changed, what's proposed, and the context plan. If the
change is notable machinery news, offer a `labs/machinery/log/` entry. If nothing
was worth keeping, say so. An empty retro is fine.
