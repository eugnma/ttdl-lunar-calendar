#!/usr/bin/env bash
#
# Used Non-POSIX features:
#   - pipefail
#   - mktemp

set -eu -o pipefail

readonly PROG='lint.sh'

usage() {
  cat <<EOF
Lint file(s).

Usage:
  ${PROG} [options] [--] <path>...

Options:
  --help                      Show this help.
  --dir-files-filter filter   Filter files from the provided directories,
                              linters will try to lint these files only if
                              possible, the choices can be one of the following:
                                - always-all: get all files even they are under
                                  version control
                                - trackable-only-if-vcs: get the trackable files
                                  only if they are under version control
                                - tracked-only-if-vcs: get the tracked files
                                  only if they are under version control
                                - new-changes-only-if-vcs: get the newly changed
                                  files only if they are under version control
                              [default: trackable-only-if-vcs].
  --no-build                  Do not build the project before linting.
EOF
}

pre_lint_file() {
  filepath="${1:?}"
  filename="${2:?}"
  extname="${3?}"
  no_build="${4:?}"

  if [ "$no_build" -eq 0 ]; then
    pre_lint_file_build "$filepath" "$filename" "$extname"
  fi
  # ADD MORE PRE-LINT SCRIPT HERE!
}

pre_lint_file_build() {
  filepath="${1:?}"
  # filename="${2:?}"
  extname="${3?}"

  # ADD THE BUILD SCRIPT HERE!
  if [ "$extname" = ".rs" ]; then
    dirname="${filepath%/*.rs}"
    cd "$dirname"
    cargo build --quiet
    cd - >/dev/null
  fi
}

post_lint_file() {
  # filepath="${1:?}"
  # filename="${2:?}"
  # extname="${3?}"
  # no_build="${4:?}"

  : # ADD THE POST-LINT SCRIPT HERE!
}

lint_file_ex() {
  filepath="${1:?}"
  # filename="${2:?}"
  extname="${3?}"

  # ADD THE LINT SCRIPT HERE IF NO SPECIAL REASON!
  if [ "$extname" = ".rs" ]; then
    cargo fmt -- --check "$filepath"
  fi
}

lint_directory_ex() {
  directory="${1:?}"
  # no_build="${2:?}"
  # dir_files_filter="${3:?}"

  # ADD THE LINT SCRIPT THAT REQUIRE DIRECTORIES HERE!
  find "$directory" -name Cargo.toml -type f | while IFS= read -r filepath; do
    dirname="${filepath%/Cargo.toml}"
    cd "$dirname"
    cargo clippy --quiet --all-targets --all-features -- -D warnings
    cd - >/dev/null
  done
}

lint() {
  path="${1:?}"
  no_build="${2:?}"
  dir_files_filter="${3:?}"

  if [ -d "$path" ]; then
    lint_directory "$path" "$no_build" "$dir_files_filter"
  elif [ -f "$path" ]; then
    lint_file "$path" "$no_build"
  else
    printf '%s\n' "$PROG: unable to lint the path: $path" >&2
    return 1
  fi
}

lint_file() {
  filepath="${1:?}"
  no_build="${2:?}"

  filename="${filepath##*/}"
  extname=''
  case "$filename" in
    *.*)
      extname=".$(printf '%s\n' "${filename##*.}" | tr '[:upper:]' '[:lower:]')"
      ;;
  esac

  # pre-lint-file
  pre_lint_file "$filepath" "$filename" "$extname" "$no_build"

  # For all files (e.g. EditorConfig)
  lint_basic "$filepath"

  # For JSON and JSONC files
  if includes -s "$extname" '.json' '.jsonc'; then
    # Prettier
    prettier_check_file "$filepath"

    return 0
  fi

  # For YAML files
  if includes -s "$extname" '.yml' '.yaml'; then
    # Prettier
    prettier_check_file "$filepath"

    return 0
  fi

  # For Dockerfile files
  if printf '%s' "$filepath" | grep -q 'Dockerfile[^/]\{0,\}$'; then
    # hadolint
    hadolint "$filepath"

    return 0
  fi

  # For Shell Scripts
  if { [ "$extname" = '.sh' ] || has_shellscript_shebang "$filepath"; }; then
    # shfmt
    # FUTURE: Use the same config for shfmt and vs-shell-format if possible
    # Google Shell Style Guide: https://git.io/JfCg8
    # Logical compounds using || and &&: https://git.io/JfC22
    shfmt -d -i 2 -bn -ci "$filepath"

    # ShellCheck
    shellcheck "$filepath"

    return 0
  fi

  # For Markdown files
  if includes -s "$extname" '.md' '.markdown'; then
    # Prettier
    prettier_check_file "$filepath"

    # markdownlint-cli
    #   - Use `>/dev/null` to avoid display the help message while linting the
    #     files from .markdownlintignore
    markdownlint "$filepath" >/dev/null

    return 0
  fi

  # For JavaScript files
  if [ "$extname" = '.js' ]; then
    # Prettier
    prettier_check_file "$filepath"

    return 0
  fi

  # Extended
  lint_file_ex "$filepath" "$filename" "$extname"

  # post-lint-file
  post_lint_file "$filepath" "$filename" "$extname" "$no_build"
}

lint_directory() {
  directory="${1:?}"
  no_build="${2:?}"
  dir_files_filter="${3:?}"

  # Lint by filepaths
  list_files "$directory" "$dir_files_filter" | while IFS= read -r filepath; do
    lint_file "$filepath" "$no_build"
  done

  # Extended
  lint_directory_ex "$directory" "$no_build" "$dir_files_filter"
}

lint_basic() {
  filepath="${1:?}"

  # editorconfig-checker
  ec "$filepath"
}

prettier_check_file() {
  filepath="${1:?}"

  if ! prettier -c "$filepath" >/dev/null; then
    printf '%s\n' "$PROG: found unformatted code by Prettier in: $filepath" >&2
    return 1
  fi
}

list_files() {
  directory="${1:?}"
  dir_files_filter="${2:?}"

  if [ "$dir_files_filter" != 'always-all' ] \
    && git -C "$directory" rev-parse --show-toplevel >/dev/null 2>&1; then
    list_files_git "$directory" "$dir_files_filter"
  else
    find "$directory" -type f
  fi
}

list_files_git() {
  directory="${1:?}"
  dir_files_filter="${2:?}"

  files_draft=''
  case "$dir_files_filter" in
    'trackable-only-if-vcs')
      files_draft="$(git ls-files -c -o --exclude-standard "$directory")"
      ;;
    'tracked-only-if-vcs')
      files_draft="$(git ls-files "$directory")"
      ;;
    'new-changes-only-if-vcs')
      files_draft="$(
        git diff --name-only -- "$directory"
        git diff --name-only --cached -- "$directory"
      )"
      ;;
  esac
  printf '%s' "$files_draft" | sort -u | while IFS= read -r filepath; do
    # Filter deleted files
    if [ ! -e "$filepath" ]; then
      continue
    fi
    printf '%s\n' "$filepath"
  done
}

has_shellscript_shebang() {
  filepath="${1:?}"

  head -n 1 -- "$filepath" | grep -Eq '^#!.*/(env +)?(sh|bash)'
}

includes() {
  optname_search="${1:?}"
  opt_search="${2?}"

  if [ "$optname_search" != '-s' ]; then
    printf '%s\n' "$PROG(includes): missing or unexpected option -s" >&2
    exit 100
  fi
  shift
  shift
  while [ $# -gt 0 ]; do
    if [ "$1" = "$opt_search" ]; then
      return 0
    fi
    shift
  done
  return 1
}

main() {
  opt_no_build=0
  opt_dir_files_filter='trackable-only-if-vcs'
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
      '--dir-files-filter')
        shift
        opt_dir_files_filter="${1:?}"
        shift
        ;;
      '--no-build')
        shift
        opt_no_build=1
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

  if ! includes -s "$opt_dir_files_filter" \
    'always-all' \
    'trackable-only-if-vcs' \
    'tracked-only-if-vcs' \
    'new-changes-only-if-vcs'; then
    printf '%s\n' \
      "$PROG: invalid argument for --dir-files-filter: $opt_dir_files_filter" \
      >&2
    return 1
  fi

  while [ $# -gt 0 ]; do
    path="$1"
    if [ ! -e "$path" ]; then
      printf '%s\n' "$PROG: no such file or directory: $filepath" >&2
      return 1
    fi
    lint "$path" "$opt_no_build" "$opt_dir_files_filter"
    shift
  done
}

# readonly UNIQUE_TEMP_DIR="$(mktemp -d)"
main "$@"
