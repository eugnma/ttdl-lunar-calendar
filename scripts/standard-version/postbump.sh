#!/usr/bin/env sh

set -eu

bash scripts/test.sh
bash scripts/lint.sh \
  --no-build \
  --dir-files-filter new-changes-only-if-vcs \
  .
