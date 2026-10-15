#!/data/data/com.termux/files/usr/bin/sh
set -eu

termux-x11 :0 >/dev/null 2>&1 &
sleep 2

exec proot-distro login --shared-tmp debian -- \
    env DISPLAY=:0 GDK_BACKEND=x11 llauncher
