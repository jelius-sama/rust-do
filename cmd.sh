#!/usr/bin/env bash
set -e

usage() {
  local exit_code=${1:-1}
  cat <<EOF
Usage: $(basename "$0") {run|build|help}

Commands:
  run     Run the application
  build   Build the application
  help    Display this help message
EOF
  exit "$exit_code"
}

# if no arguments, show help and exit with error
if [ $# -ne 1 ]; then
  usage
fi

case "$1" in
  run)
    ./target/x86_64-unknown-linux-musl/release/rust-do
    ;;
  build)
    cargo build --target=x86_64-unknown-linux-musl --release
    ;;
  help)
    usage 0
    ;;
  *)
    # unrecognized sub‑command → show help + error
    usage
    ;;
esac
