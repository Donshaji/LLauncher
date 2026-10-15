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

Inside Debian, install the GTK4 development libraries and build tools:

```bash
apt update
apt install build-essential pkg-config libgtk-4-dev wmctrl xdotool x11-xserver-utils
```

Install Rust with rustup (the project uses Rust 2024 edition, which requires
Rust 1.85 or newer), then load Cargo into the shell:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

## One-tap startup

Clone this project inside Debian at `/root/LLauncher` (or edit
`PROJECT_DIR` in `scripts/start-llauncher-termux.sh` to match its location).
After setting up the Debian dependencies and Rust below, install the script as
a Termux:Widget shortcut from the Termux shell:

```bash
mkdir -p "$HOME/.shortcuts"
proot-distro login debian -- cat /root/LLauncher/scripts/start-llauncher-termux.sh \
  > "$HOME/.shortcuts/llauncher"
chmod +x "$HOME/.shortcuts/llauncher"
```

Add the Termux:Widget to the Android home screen and tap **llauncher** to start
Termux:X11, enter Debian, and run LLauncher. The first launch compiles the
release build and can take a while; later launches reuse it.

## Manual build and run

You can also start the X11 server in Termux and run these commands inside
Debian. The usual display is `:0`; change `DISPLAY` if needed.

```bash
cargo build
DISPLAY=:0 GDK_BACKEND=x11 cargo run
```

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
