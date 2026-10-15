# LLauncher

A launcher and desktop environment for Linux ARM64 smartphone form factor devices.

## Status
- [x] Initial project setup
- [x] GTK4 application window scaffold
- [ ] Core launcher UI
- [ ] App management
- [ ] System integration (display/input)
- [ ] Wayland/Compositor integration
- [ ] Touch/input handling
- [ ] ARM64 optimizations

## Requirements

This project targets 64-bit ARM Debian running inside Termux:proot and displays
its GTK4 window through the Termux:X11 Android app. Install the Termux:X11
Android companion app, then install and start its server from the Termux shell:

```bash
pkg install x11-repo
pkg install termux-x11-nightly
termux-x11 :0 &
```

Inside the Debian proot, install the GTK4 development libraries and build tools:

```bash
apt update
apt install build-essential pkg-config libgtk-4-dev
```

Install Rust with rustup (the project uses Rust 2024 edition, which requires
Rust 1.85 or newer), then load Cargo into the shell:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

## Build and run

Start the Termux:X11 server from Termux, then run these commands inside Debian.
The usual display is `:0`; change `DISPLAY` if the server was started on a
different display.

```bash
cargo build
DISPLAY=:0 GDK_BACKEND=x11 cargo run
```

The current executable opens a simple portrait-oriented GTK4 window. The full
launcher UI and system integration are not implemented yet.
