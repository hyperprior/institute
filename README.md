# Hyperprior Institute

A lab that builds labs that builds labs. Live at **[hyperprior.institute](https://hyperprior.institute)**.

The institute is run by hyperprior, its scientist. Ideas start as sparks, become
sketches when they run and labs when they stick, and sometimes spin out as
libraries. The rest get a write-up and a decent burial. The site is rendered
straight from this repo by `hp`, the machinery (lab #0), using no framework and no
JavaScript.

```sh
just hp spark "an idea"            # record a spark
just hp new sketch my-idea         # scaffold labs/my-idea and run it: cargo run -p my-idea
just hp promote my-idea            # sketch → lab → library
just hp bury my-idea               # with an epitaph
just serve                         # preview at http://localhost:8000
just check                         # fmt, clippy, tests
```

See [CLAUDE.md](CLAUDE.md) for the institute's conventions.

## Layout

```
labs/<name>/      a project: lab.toml, log/, figures/, src/ (a cargo crate)
labs/machinery/   lab #0: the hp CLI and site generator
sparks.toml       the idea list
about.md          the home page intro
.claude/skills/   lab-tech skills for Claude Code
```

## Deploying

Every push to `main` runs the tests, builds the site and deploys it to the
Cloudflare Pages project `hyperprior-institute`, served at hyperprior.institute
(`.github/workflows/deploy.yml`). CI authenticates with the repo secrets
`CLOUDFLARE_API_TOKEN` (Pages edit) and `CLOUDFLARE_ACCOUNT_ID`.
