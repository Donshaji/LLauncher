# LLauncher

A launcher and desktop environment for Linux ARM64 smartphone form factor devices.

## Status
- [x] Initial project setup
- [x] GTK4 application window scaffold
- [x] Touch-friendly navigation bar
- [ ] Core launcher UI
- [ ] App management
- [ ] System integration (display/input)
- [ ] Wayland/Compositor integration
- [ ] Touch/input handling
- [ ] ARM64 optimizations

## Requirements

This project targets 64-bit ARM Debian running inside Termux:proot and displays
its GTK4 window through the Termux:X11 Android app.

Install the Termux:X11 Android companion app, then install the Termux packages
from the Termux shell:

```bash
pkg install x11-repo
pkg install termux-x11-nightly
pkg install proot-distro
pkg install termux-widget
```

## Install a release

Download the latest ARM64 release and install the runtime dependencies inside
the Debian proot:

```bash
apt update
apt install ca-certificates curl libgtk-4-1 wmctrl xdotool x11-xserver-utils
cd /tmp
curl -fL \
  https://github.com/Donshaji/LLauncher/releases/latest/download/llauncher-aarch64-unknown-linux-gnu.tar.gz \
  -o llauncher-aarch64-unknown-linux-gnu.tar.gz
curl -fL \
  https://github.com/Donshaji/LLauncher/releases/latest/download/llauncher-aarch64-unknown-linux-gnu.tar.gz.sha256 \
  -o llauncher-aarch64-unknown-linux-gnu.tar.gz.sha256
sha256sum -c llauncher-aarch64-unknown-linux-gnu.tar.gz.sha256
tar -xzf llauncher-aarch64-unknown-linux-gnu.tar.gz
install -m 755 llauncher /usr/local/bin/llauncher
```

The binary runs on 64-bit ARM Debian (AArch64) with GTK4 installed. No Rust
toolchain is needed on the phone when installing a release.

## One-tap startup

After installing the binary and Termux packages above, copy the startup script
to the Termux:Widget shortcuts directory from the Termux shell:

```bash
mkdir -p "$HOME/.shortcuts"
curl -fL \
  https://raw.githubusercontent.com/Donshaji/LLauncher/main/scripts/start-llauncher-termux.sh \
  > "$HOME/.shortcuts/llauncher"
chmod +x "$HOME/.shortcuts/llauncher"
```

Add the Termux:Widget to the Android home screen and tap **llauncher** to start
Termux:X11, enter Debian with shared temporary storage, and run LLauncher.

## Build from source

For development, install the GTK4 development packages in Debian:

```bash
apt install build-essential pkg-config libgtk-4-dev wmctrl xdotool x11-xserver-utils
```

Install Rust 1.85 or newer with rustup, then build and run inside Debian:

```bash
cargo build
DISPLAY=:0 GDK_BACKEND=x11 cargo run
```

To publish a release, push a version tag such as `v0.1.0`. GitHub Actions builds
the AArch64 Linux binary on Debian Bookworm, packages it, calculates a SHA-256
checksum, and publishes both files on the GitHub Release.

The bottom navigation buttons work as follows:

- **Recent Apps** lists other X11 windows. Select one to make it the target for
  **Back** and **Close App**.
- **Home** returns to the LLauncher window.
- **Back** activates the selected window and sends it Escape.
- **Rotate** toggles the connected X11 display between left and right rotation.
- **Close App** asks the selected window to close.

These actions rely on `wmctrl`, `xdotool`, and `xrandr` being available in the
Debian proot. Display rotation depends on Termux:X11 and the device supporting
XRandR rotation; unsupported actions display an error. The full launcher home
screen, app management, and system integration are still under development.
