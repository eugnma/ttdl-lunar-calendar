#!/usr/bin/env sh

set -eu

shebang-run scripts/lint.sh CHANGELOG.md

# ADD BUILD ARTIFACTS TO THE RELEASE COMMIT HERE IF ANY CHANGE!
git add Cargo.lock
