#!/usr/bin/env bash
#
# Used Non-POSIX features:
#   - pipefail
#   - mktemp
#   - node: https://github.com/nodejs/node
#   - commonmark: https://github.com/commonmark/commonmark.js
#   - prettier: https://github.com/prettier/prettier
#   - tar and xz on systems other than Windows
#   - 7z on Windows

set -eu -o pipefail

readonly PROG='prepare-release-asset.sh'
readonly VERSION='0.1.0'

version() {
  printf '%s\n' "$PROG v$VERSION"
}

usage() {
  cat <<EOF
Prepare release asset.

Usage:
  ${PROG} [options]

Options:
  --help              Show this help.
  --version           Show version.
  --base-url URL      The base url for local unreachable links.
  --output-dir PATH   The output directory [default: .].
EOF
}

get_program_name() {
  # CHANGE THE SCRIPT HERE TO GET THE PROGRAM NAME!
  printf '%s\n' 'dev-container-template-rust'
}

prepare_release_artifacts() {
  # CHANGE THE SCRIPT HERE TO PREPARE RELEASE ARTIFACTS!
  cargo build --release
}

cherry_copy_release_artifacts() {
  dest_dir="${1:?}"

  # CHANGE THE SCRIPT HERE TO CHERRY COPY RELEASE ARTIFACTS!
  program_name="$(get_program_name)"
  if [ "$(get_os)" = 'windows' ]; then
    cp "target/release/$program_name.exe" "$dest_dir/$program_name.exe"
  else
    cp "target/release/$program_name" "$dest_dir/$program_name"
  fi
}

get_release_asset_filename_without_ext() {
  # CHANGE THE SCRIPT HERE TO GET RELEASE ASSET FILENAME WITHOUT EXTENSION!
  program_name="$(get_program_name)"
  rust_os="$(rustc --print cfg | grep '^target_os=' | cut -d '"' -f 2)"
  rust_arch="$(rustc --print cfg | grep '^target_arch=' | cut -d '"' -f 2)"
  printf '%s\n' "$program_name-$rust_os-$rust_arch"
}

prepare_release_asset() {
  base_url="${1:?}"
  output_dir="${2:?}"

  prepare_release_artifacts
  tempdir="$(mktemp -d)"
  generate_user_friendly_readme "$base_url" "$tempdir"
  cherry_copy_release_artifacts "$tempdir"
  for path in LICENSE*; do
    if [ -e "$path" ]; then
      cp "$path" "$tempdir"
    fi
  done
  release_asset_filename_without_ext="$(get_release_asset_filename_without_ext)"
  if [ "$(get_os)" = 'windows' ]; then
    7z a \
      -tzip \
      "$output_dir/$release_asset_filename_without_ext.zip" \
      "$tempdir/"*
  else
    tar -cJ \
      -f "$output_dir/$release_asset_filename_without_ext.tar.xz" \
      -C "$tempdir" \
      .
  fi
}

generate_user_friendly_readme() {
  base_url="${1:?}"
  dest_dir="${2:?}"

  optional_base_url=''
  if [ -n "$base_url" ]; then
    optional_base_url="$base_url/"
  fi
  fill_html_body "$(commonmark README.md)" \
    | node 'scripts/release/replace-html-link.js' \
      "CODE_OF_CONDUCT.md -> ${optional_base_url}CODE_OF_CONDUCT.md" \
      "CONTRIBUTING.md -> ${optional_base_url}CONTRIBUTING.md" \
    | prettier --parser html \
      >"$dest_dir/README.html"
}

fill_html_body() {
  body="${1?}"
  cat <<EOF
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8" />
    </head>
    <body>
        $body
    </body>
</html>
EOF
}

get_os() {
  # https://en.wikipedia.org/wiki/Uname
  case "$(uname -s)" in
    'Linux') printf '%s\n' 'linux' ;;
    'Darwin') printf '%s\n' 'macos' ;;
    'MINGW'* | 'CYGWIN'*) printf '%s\n' 'windows' ;;
    *) printf '%s\n' 'unknown' ;;
  esac
}

main() {
  opt_base_dir='.'
  opt_output_dir='.'
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
      '--base-url')
        shift
        opt_base_dir="${1:?}"
        shift
        ;;
      '--output-dir')
        shift
        opt_output_dir="${1:?}"
        shift
        ;;
      '-'*)
        printf '%s\n' "$PROG: unknown option: $1" >&2
        return 1
        ;;
      *) break ;;
    esac
  done
  normalized_base_url="${opt_base_dir%/}"
  normalized_output_dir="${opt_output_dir%/}"
  prepare_release_asset "$normalized_base_url" "$normalized_output_dir"
}

main "$@"
