# Teaching the lab to learn

A lab that builds labs should also get better at building them. As of today the
institute has a self-learning loop in three parts:

- **Remember.** Corrections and preferences are saved to the agent's memory the
  moment they happen, so the next session starts out knowing them.
- **Improve.** A `retro` skill reviews each substantial session for corrections,
  friction and steps repeated by hand, and turns them into better rules, skills
  or `hp` commands. Small fixes get applied. Bigger ones get proposed.
- **Heal.** When a documented step breaks, the fix goes into the doc or tool that
  caused it, not into a one-off workaround.

The loop shouldn't depend on anyone remembering to run it, so a small hook taps
the agent on the shoulder once per session after enough work has happened. It's
written in shell rather than Rust because it has to keep working while `hp` itself
is mid-surgery.

Plans now go through lavish first, as annotatable pages, before anything gets built.
