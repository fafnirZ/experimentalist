# Quadtree visualiser.
gfx-rs/wgpu for visualisation

https://sotrh.github.io/learn-wgpu/beginner/tutorial1-window/#the-code



# IMPORTANT PRE-requisites for WSL2
```
apt install libxkbcommon-x11-0
```

winit will try to use wayland, prevent this.
```
export WAYLAND_DISPLAY=
RUSTFLAGS="--cfg x11_platform" cargo run
```

this is because theres a bug associated with 
https://github.com/PolyMeilex/sctk-adwaita/blob/master/src/theme.rs#L24

which assumes linux environment and tries to read some kind of config which doesnt exist in WSL2

https://github.com/rust-windowing/winit/blob/ed4d70fdd415aa4185c7f6e4e8dbd1052e8dca65/src/platform_impl/linux/mod.rs#L147
inside winit, so long as the env var `WAYLAND_DISPLAY` is detected, it will take wayland as priority over all else.
even the RUSTFLAGS="--cfg x11_platform"

technically I don't think you need the RUSTFLAGS="" anymore in this, but I'm just going to leave it here for documentation 
since this is how I know in the future how to manually set cfg flags in rust :)

this is a learning exercise for me afterall.