---
name: propose-extraction
description: Look for code repeated across Hyperprior Institute labs and propose extracting it into shared machinery. Use when hyperprior asks what should become shared, or when you notice yourself copying code between labs.
---

# Propose extraction

The rule of three: code becomes shared machinery only once it lives in three places.

1. Search `labs/*/src` for duplicated or near-duplicated functions, types and patterns.
2. Report each candidate with its call sites (at least three), what a shared
   API would look like, and the cost of extracting it now versus later.
3. Propose, don't do. On approval:
   - Create `crates/<name>` and add `"crates/*"` to the workspace `members`
     in the root `Cargo.toml` if it isn't there yet.
   - Point the call sites at it and delete the copies.
   - Log the extraction in `labs/machinery/log/`, because extractions are machinery news.
