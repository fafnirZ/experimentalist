#!/bin/bash

# removes that env var to force it to use X11
# https://github.com/rust-windowing/winit/blob/ed4d70fdd415aa4185c7f6e4e8dbd1052e8dca65/src/platform_impl/linux/mod.rs#L147
# due to bug associated with WSL in
# https://github.com/PolyMeilex/sctk-adwaita/blob/master/src/theme.rs#L24
export WAYLAND_DISPLAY=
RUSTFLAGS="--cfg x11_platform" cargo run