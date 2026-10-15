#!/usr/bin/env bash
# Stop hook: once per session, after substantial work, ask Claude to run the retro.
# Shell rather than Rust on purpose: a hook must work even while hp is mid-edit
# and doesn't compile.
set -euo pipefail

input=$(cat)
[ "$(jq -r '.stop_hook_active // false' <<<"$input")" = "true" ] && exit 0

session=$(jq -r '.session_id // empty' <<<"$input")
transcript=$(jq -r '.transcript_path // empty' <<<"$input")
[ -n "$session" ] && [ -f "$transcript" ] || exit 0

marker="${TMPDIR:-/tmp}/hyperprior-retro/$session"
[ -e "$marker" ] && exit 0

# "Substantial" = enough tool use that there's likely something to learn from.
tool_uses=$(grep -o '"type":"tool_use"' "$transcript" | wc -l | tr -d ' ')
[ "$tool_uses" -ge "${RETRO_NUDGE_THRESHOLD:-30}" ] || exit 0

# Skip if the retro skill already ran this session.
grep -q '"skill":"retro"' "$transcript" && exit 0

mkdir -p "$(dirname "$marker")" && touch "$marker"
jq -n '{decision: "block", reason: "Self-learning nudge (once per session): this session has done substantial work. If this is a natural stopping point, run the `retro` skill now. If hyperprior is mid-task, finish the current step first and run it at the next stopping point."}'
