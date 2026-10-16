# LLauncher

Launcher/Desktop environment for Linux ARM64 smartphone form factor.

## Installation (Prebuilt Binaries)

The easiest way to install LLauncher is to download a prebuilt binary from the [Releases](https://github.com/don/LLauncher/releases) page. This requires no build dependencies or Rust toolchain.

### For Linux ARM64 (aarch64)

1. Download the latest `llauncher-aarch64-linux-gnu.tar.gz` or `llauncher-aarch64-linux-gnu` from the Releases page.
2. Extract and make it executable:
   ```bash
   tar -xzf llauncher-aarch64-linux-gnu.tar.gz
   chmod +x llauncher-aarch64-linux-gnu
   ./llauncher-aarch64-linux-gnu
   ```
3. (Optional) Move it to your PATH:
   ```bash
   sudo mv llauncher-aarch64-linux-gnu /usr/local/bin/llauncher
   llauncher
   ```

## Building from Source

If you want to build locally:

```bash
cargo build --release
```

The binary will be at `target/release/llauncher`. This requires the Rust toolchain to be installed.