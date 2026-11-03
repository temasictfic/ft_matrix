#!/usr/bin/env bash
# Builds the ex14 projection matrix and opens it in the subject's display program.
#
#   ./display.sh                     # fov 70°, ratio 1, near 0.1, far 100
#   ./display.sh 90 1 0.5 50         # fov_degrees ratio near far
#
# Why not just ./display? The program moves the camera with the mouse. On every
# cursor event it does `yaw += (x - 500) * 0.01`, `pitch -= (y - 500) * 0.01`,
# then warps the cursor back to (500, 500). Two things break that on a desktop
# Linux machine and leave the window black (the camera ends up looking at the
# floor):
#   1. HiDPI scaling (e.g. GNOME at 125%): the warp target (500, 500) is in
#      logical pixels but events report physical ones, so every warp reads as a
#      125 px jump and the camera spins forever. Fix: WINIT_X11_SCALE_FACTOR=1.
#   2. The very first event reports wherever the cursor happens to be, which is
#      one big jump. Fix: put the cursor at the window's centre before it opens.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DIR="$ROOT/matrix_display"
TITLE="42AI - Projection Matrix Display"

[[ -x "$DIR/display" ]] || tar xzf "$ROOT/display_linux.tar.gz" -C "$ROOT"

(cd "$DIR" && cargo run --quiet --manifest-path "$ROOT/Cargo.toml" --bin ex14 -- "$@")

export WINIT_X11_SCALE_FACTOR=1
export WINIT_UNIX_BACKEND=x11 # XWayland on Wayland sessions, so the cursor warp works

warp_pointer() {
    python3 - "$1" "$2" <<'EOF'
import ctypes, sys
x11 = ctypes.CDLL("libX11.so.6")
x11.XOpenDisplay.restype = ctypes.c_void_p
x11.XDefaultRootWindow.argtypes = [ctypes.c_void_p]
x11.XDefaultRootWindow.restype = ctypes.c_ulong
x11.XWarpPointer.argtypes = [ctypes.c_void_p, ctypes.c_ulong, ctypes.c_ulong,
                             ctypes.c_int, ctypes.c_int, ctypes.c_uint, ctypes.c_uint,
                             ctypes.c_int, ctypes.c_int]
x11.XFlush.argtypes = x11.XCloseDisplay.argtypes = [ctypes.c_void_p]
display = x11.XOpenDisplay(None)
if not display:
    sys.exit(1)
x11.XWarpPointer(display, 0, x11.XDefaultRootWindow(display), 0, 0, 0, 0,
                 int(sys.argv[1]), int(sys.argv[2]))
x11.XFlush(display)
x11.XCloseDisplay(display)
EOF
}

# Prints the centre of the display window, waiting up to ~5 s for it to appear.
window_centre() {
    for _ in $(seq 100); do
        if xwininfo -name "$TITLE" >/dev/null 2>&1; then
            xwininfo -name "$TITLE" | awk '
                /Absolute upper-left X/ { x = $4 } /Absolute upper-left Y/ { y = $4 }
                /Width:/ { w = $2 } /Height:/ { h = $2 }
                END { print int(x + w / 2), int(y + h / 2) }'
            return 0
        fi
        sleep 0.05
    done
    return 1
}

cd "$DIR"
if command -v python3 >/dev/null && command -v xwininfo >/dev/null; then
    # The window manager places the window at the same spot each time, so open
    # it once to learn where, then reopen it with the cursor already there.
    ./display >/dev/null 2>&1 &
    probe=$!
    centre="$(window_centre || true)"
    kill "$probe" 2>/dev/null || true
    wait "$probe" 2>/dev/null || true
    if [[ -n "$centre" ]]; then
        # shellcheck disable=SC2086
        warp_pointer $centre || true
    fi
else
    echo "note: install python3 and x11-utils (xwininfo) so the cursor can be centred;" >&2
    echo "      otherwise start with the mouse in the middle of the window." >&2
fi
exec ./display
