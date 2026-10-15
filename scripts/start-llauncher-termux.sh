#!/data/data/com.termux/files/usr/bin/sh
set -eu

PROJECT_DIR=/root/LLauncher

termux-x11 :0 >/dev/null 2>&1 &
sleep 2

exec proot-distro login debian -- \
    bash -lc 'cd "$1" && exec env DISPLAY=:0 GDK_BACKEND=x11 cargo run --release' \
    llauncher "$PROJECT_DIR"
