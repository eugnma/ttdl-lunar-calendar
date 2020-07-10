#!/usr/bin/env sh

set -eu

readonly PROG='lint.sh'

usage() {
  cat <<EOF
Run tests.

Usage:
  ${PROG} [options]

Options:
  --help                Show this help.
  --release             Run tests in in release mode.
EOF
}

test_debug() {
  # ADD THE BUILD AND TEST SCRIPT FOR DEBUG MODE HERE!
  cargo test
}

test_release() {
  # ADD THE BUILD AND TEST SCRIPT FOR RELEASE MODE HERE!
  cargo test --release
}

main() {
  opt_release=0
  while [ $# -gt 0 ]; do
    case "$1" in
      '--help')
        usage
        return 0
        ;;
      '--release')
        shift
        opt_release=1
        ;;
      '-'*)
        printf '%s\n' "$PROG: unknown option: $1" >&2
        return 1
        ;;
      *) break ;;
    esac
  done
  if [ "$opt_release" -eq 0 ]; then
    test_debug
  else
    test_release
  fi
}

main "$@"
