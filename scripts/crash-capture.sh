#!/usr/bin/env bash
# A crash of the shipped binary in CI, captured (W-169): `build <key>`'s
# `codelaw test` / `codelaw rules lint` loop segfaulted now and then, exit 139
# with nothing printed, and passed on a rerun. This keeps the evidence.
#
#   scripts/crash-capture.sh arm DIR
#       before the binary runs: the kernel writes every core into DIR
#       (Linux: kernel.core_pattern; macOS: kern.corefile; Windows: WER's
#       LocalDumps for codelaw.exe). The step that runs the binary must also
#       say `ulimit -c unlimited` — a limit is the shell's, not this script's.
#   scripts/crash-capture.sh collect DIR BINARY TARGET [MATURIN FLAGS…]
#       after a failure: DIR gets the shipped binary, a symbolized twin and
#       backtrace.txt — every thread's stack, from gdb (Linux, glibc and musl
#       alike: a static musl binary's core reads the same) or lldb (macOS),
#       plus macOS's own crash report. Windows keeps the minidump only.
#
# The shipped binary is stripped (packaging/python/pyproject.toml `strip`), so
# its frames are bare addresses. The twin is the SAME maturin build with
# `strip = false` — same toolchain, target, flags and sources, so the same
# code — built here only on a crash and never shipped; backtrace.txt says
# whether its .text is byte-identical to the shipped one (Linux), so a reader
# knows whether the names can be trusted. Debug info never enters a package.
set -euo pipefail
cmd="${1:?usage: crash-capture.sh arm DIR | collect DIR BINARY TARGET [MATURIN FLAGS…]}"
dir="${2:?a directory for the cores}"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
case "$(uname -s)" in
  Linux) os=linux ;;
  Darwin) os=macos ;;
  *) os=windows ;;
esac
mkdir -p "$dir"
dir="$(cd "$dir" && pwd)"

arm() {
  case "$os" in
    linux)
      sudo sysctl -q -w kernel.core_pattern="$dir/core.%e.%p.%t"
      sudo sysctl -q -w kernel.core_uses_pid=1 ;;
    macos)
      sudo sysctl -q -w kern.coredump=1
      sudo sysctl -q -w kern.corefile="$dir/core.%P" ;;
    windows)
      local key='HKLM\SOFTWARE\Microsoft\Windows\Windows Error Reporting\LocalDumps\codelaw.exe'
      MSYS_NO_PATHCONV=1 reg add "$key" /v DumpFolder /t REG_EXPAND_SZ /d "$(cygpath -w "$dir")" /f >/dev/null
      MSYS_NO_PATHCONV=1 reg add "$key" /v DumpType /t REG_DWORD /d 2 /f >/dev/null
      MSYS_NO_PATHCONV=1 reg add "$key" /v DumpCount /t REG_DWORD /d 10 /f >/dev/null ;;
  esac
  echo "✔ a crash of codelaw writes its core into $dir"
}

# The same maturin build as the shipped wheel, unstripped, into $dir/twin.
twin() {
  local target="$1"; shift
  local v; v="$(sed -n 's/^channel = "\(.*\)"/\1/p' "$root/rust-toolchain.toml")"
  rustup toolchain install "$v" --profile minimal --target "$target" --no-self-update >/dev/null 2>&1 || true
  command -v maturin >/dev/null || pip install --disable-pip-version-check -q "maturin[zig]==${MATURIN_VERSION:?}"
  (cd "$root" && python scripts/package.py prepare-wheel >/dev/null)
  local py="$root/packaging/python/pyproject.toml"
  cp "$py" "$dir/pyproject.toml.shipped"
  sed 's/^strip = true$/strip = false/' "$dir/pyproject.toml.shipped" > "$py"
  local rc=0
  (cd "$root/packaging/python" && maturin build --release --locked --target "$target" "$@" --out "$dir/twin-wheel") || rc=$?
  cp "$dir/pyproject.toml.shipped" "$py" && rm "$dir/pyproject.toml.shipped"
  [ "$rc" = 0 ] || return "$rc"
  python "$root/scripts/package.py" extract "$dir"/twin-wheel/*.whl "$dir/twin" >/dev/null
  rm -rf "$dir/twin-wheel"
}

collect() {
  local bin="${3:?the binary that ran}" target="${4:?the rust target}"; shift 4
  local report="$dir/backtrace.txt"
  shopt -s nullglob
  if [ "$os" = macos ]; then
    for r in "$HOME"/Library/Logs/DiagnosticReports/codelaw*; do cp "$r" "$dir/"; done
  fi
  local cores=("$dir"/core.* "$dir"/*.dmp)
  if [ "${#cores[@]}" = 0 ]; then
    echo "no core was written in $dir: the failure was not a crash, or the kernel refused the dump" | tee "$report"
    return 0
  fi
  cp "$bin" "$dir/"
  local shipped; shipped="$dir/$(basename "$bin")"
  { echo "# W-169 crash capture — $(uname -sm), target $target, commit ${GITHUB_SHA:-?}"; echo "cores: ${cores[*]##*/}"; } > "$report"
  if [ "$os" = windows ]; then
    echo "Windows: the minidumps are here; open one in WinDbg with the binary beside it" >> "$report"
    return 0
  fi
  local sym="$shipped"
  if twin "$target" "$@" >> "$dir/twin-build.log" 2>&1; then
    sym="$dir/twin/$(basename "$bin")"
    if [ "$os" = linux ]; then
      objcopy -O binary --only-section=.text "$shipped" "$dir/.a" && objcopy -O binary --only-section=.text "$sym" "$dir/.b"
      if cmp -s "$dir/.a" "$dir/.b"; then echo "twin: .text byte-identical to the shipped binary — its names are the crash's" >> "$report"
      else echo "twin: .text DIFFERS from the shipped binary — read its names as a guide only" >> "$report"; fi
      rm -f "$dir/.a" "$dir/.b"
    fi
  else
    echo "twin: the unstripped build failed (twin-build.log); frames below are addresses only" >> "$report"
  fi
  for c in "$dir"/core.*; do
    echo; echo "## $(basename "$c")"
    case "$os" in
      linux)
        command -v gdb >/dev/null || { sudo apt-get -qq update && sudo apt-get -qq install -y gdb; } >/dev/null 2>&1
        # gdb's own convenience variables, not the shell's:
        # shellcheck disable=SC2016
        gdb -batch -nx -ex 'p $_siginfo' -ex 'info registers' -ex 'x/6i $pc' \
          -ex 'thread apply all bt' "$sym" "$c" 2>&1 ;;
      macos)
        lldb --batch -c "$c" "$sym" -o 'register read' -o 'thread backtrace all' 2>&1 ;;
    esac
  done >> "$report"
  echo "✔ $report"
}

case "$cmd" in
  arm) arm ;;
  collect) collect "$@" ;;
  *) echo "crash-capture.sh: unknown command $cmd" >&2; exit 2 ;;
esac
