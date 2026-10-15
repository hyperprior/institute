# Run the institute CLI: `just hp spark "an idea"`
hp *args:
    cargo run -q -p hp -- {{args}}

build:
    cargo run -q -p hp -- build

serve:
    cargo run -q -p hp -- serve

test:
    cargo test --workspace

check:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace

# Put `hp` on your PATH
install:
    cargo install --path labs/machinery

# Check, push, wait for the deploy, and confirm the live site answers
ship: check
    #!/usr/bin/env bash
    set -euo pipefail
    repo=hyperprior/institute
    git push
    sha=$(git rev-parse HEAD)
    run=""
    for _ in $(seq 1 30); do
        run=$(gh run list -R "$repo" --commit "$sha" --workflow deploy --json databaseId --jq '.[0].databaseId // empty')
        [ -n "$run" ] && break
        sleep 2
    done
    [ -n "$run" ] || { echo "no deploy run found for ${sha:0:7}"; exit 1; }
    echo "deploying ${sha:0:7} (run $run)…"
    gh run watch "$run" -R "$repo" --exit-status --interval 10 > /dev/null || { echo "deploy failed: gh run view $run -R $repo --log-failed"; exit 1; }
    ip=$(dig +short hyperprior.institute @1.1.1.1 | head -1)
    code=000
    for _ in $(seq 1 15); do
        code=$(curl -s -o /dev/null -w '%{http_code}' --resolve "hyperprior.institute:443:$ip" https://hyperprior.institute/)
        [ "$code" = 200 ] && { echo "shipped ${sha:0:7} → https://hyperprior.institute"; exit 0; }
        sleep 2
    done
    echo "deploy succeeded but the site returned $code"
    exit 1
