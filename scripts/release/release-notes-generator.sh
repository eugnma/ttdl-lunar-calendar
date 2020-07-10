#!/usr/bin/env sh

set -eu

readonly PROG='release-notes-generator.sh'
readonly VERSION='0.1.0'
readonly BRE_VERSION_PREFIX='^#\{2,3\} \[\{0,1\}'
readonly BRE_VERSION="${BRE_VERSION_PREFIX}[0-9]*\.[0-9]*\.[0-9]*"

version() {
  printf '%s\n' "$PROG v$VERSION"
}

usage() {
  cat <<EOF
Generate release notes for a specific version.

Usage:
  ${PROG} [options] [--] <version>

Options:
  --help      Show this help.
  --version   Show version.
EOF
}

generate() {
  version="${1:?}"

  specific_bre_version_core="$(printf '%s' "$version" | sed 's/\./\\\./g')"
  specific_bre_version="${BRE_VERSION_PREFIX}$specific_bre_version_core"
  found=0
  while IFS= read -r line; do
    if [ "$found" -eq 0 ]; then
      if printf '%s\n' "$line" | grep -q "$specific_bre_version"; then
        found=1
        printf '%s\n' "$line"
      fi
    else
      if printf '%s\n' "$line" | grep -q "$BRE_VERSION"; then
        break
      fi
      printf '%s\n' "$line"
    fi
  done <'CHANGELOG.md'
  if [ "$found" -eq 0 ]; then
    return 1
  fi
}

main() {
  while [ $# -gt 0 ]; do
    case "$1" in
      '--')
        shift
        break
        ;;
      '--help')
        usage
        return 0
        ;;
      '--version')
        version
        return 0
        ;;
      '-'*)
        printf '%s\n' "$PROG: unknown option: $1" >&2
        return 1
        ;;
      *) break ;;
    esac
  done
  if [ $# -eq 0 ]; then
    usage
    return 1
  fi
  generate "$@"
}

main "$@"
