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
- **Verbose I/O.** What the session wrote and read, not only what it loaded.
  Replies: preamble, restating the ask, prose that retells a table, hedges,
  closing summaries nobody needed. Tool calls: output that a filter, `-q` or
  `| tail` would have cut. Instructions: skills and CLAUDE.md that are wordy
  themselves or ask for wordy output (long report templates, mandatory
  sections). The fix is the institute's own caveman, built by hand: a terse
  house-style rule in CLAUDE.md, a tighter template in a skill, a quieter
  `hp` flag. Each proposal names the pattern, shows a before → after from this
  session, and estimates the saving. Measure with:
  ```sh
  jq -s '[.[] | .message.usage.output_tokens? // empty] | add' "$f"   # tokens written
  jq -r 'select(.type=="user") | .message.content | arrays | .[]
    | select(.type=="tool_result") | (.content | tostring | length / 4 | floor)' "$f" \
    | sort -rn | head -5                                               # biggest tool results, est.
  ```
- **Numbers.** Call `mcp__ccd_session_mgmt__get_usage` (session `self`). It
  returns tokens per category (MCP tools, system tools, skills, memory, agents).
  Compare them with the last retro's numbers in the `context-trim` memory. It
  merges loaded and deferred tools, so use it for trends, not to match `/context`.
  It only exists in the desktop app. In the CLI or a cloud session, get the
  total from the newest transcript (input + cache read + cache creation of the
  last assistant turn):
  ```sh
  f=$(ls -t ~/.claude/projects/$(pwd | tr / -)/*.jsonl | head -1)
  jq -r 'select(.type=="assistant" and .message.usage) | .message.usage
    | .input_tokens + .cache_read_input_tokens + .cache_creation_input_tokens' "$f" 2>/dev/null | tail -1
  ```
  Use jq, not grep: tool output lines mention "usage" too, and some lines have
  raw control characters that break a naive JSON parse. That has no categories,
  so ask hyperprior for `/context` if you need them.

Present the plan as a table: each killer, its size in tokens, its share of the
session's total context as a percentage, the cut, and its scope (repo,
project-local `settings.local.json`, or global). State the total first, and end
with a row summing the proposed cuts (tokens and %). Without `/context` numbers,
estimate a tool's size as the characters of its description plus schema divided
by 4, and mark it "est.". To remove a tool from this repo's context, add a
whole-tool deny rule (`Artifact`, `mcp__<server>`) to `settings.local.json`; for
skills, also set `skillOverrides`. Project-local and repo
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
