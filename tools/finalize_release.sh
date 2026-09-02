#!/usr/bin/env bash
set -euo pipefail

REL="$(cd "$(dirname "$0")/.." && pwd)"

cd "$REL"

rm -f SHA256SUMS

find . -type f \
    -not -path './.git/*' \
    -not -path './_runs/*' \
    -not -path './SHA256SUMS' \
    -print0 |
    sort -z |
    xargs -0 sha256sum > SHA256SUMS

if [ ! -d .git ]; then
    git init -b main
fi

git add -A

if ! git diff --cached --quiet; then
    git commit -m "Redumb DUMB v3preview alpha"
fi

git tag -f duv3pre-alpha0

echo
echo "HEAD=$(git rev-parse HEAD)"
echo "TAG=$(git rev-list -n1 duv3pre-alpha0)"
echo
git status --short
