#!/usr/bin/env sh

set -eu

shebang-run scripts/test.sh
shebang-run scripts/lint.sh \
  --no-build \
  --dir-files-filter new-changes-only-if-vcs \
  .
