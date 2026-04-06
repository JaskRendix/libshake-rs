# libShake

A cross‑platform haptic library written in Rust.

## Overview

libShake provides a unified API for force‑feedback devices.  
It exposes rumble, periodic, constant, and ramp effects through a platform‑agnostic interface.

The library includes:

- **Linux backend** using the evdev force‑feedback subsystem  
- **Mock backend** for development and testing  
- A shared `Device` API  
- An `Effect` model with envelopes, timing, and direction

The mock backend provides:

- Timeline tracking  
- Rumble mixing with gain  
- Periodic and ramp ASCII oscilloscopes  
- Gain and autocenter state  
- A simple profiler  
- Log export

## Origins

The original libShake was written in C and targeted Linux force‑feedback.  
This Rust version is a clean rewrite with a revised API and backend layout.

Original project: https://github.com/zear/libShake

## Installation

### Linux

Build with Cargo:

```sh
cargo build --release
```

The Linux backend uses `nix` and `libc` to access `/dev/input/event*`.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
shake = { path = "path/to/libshake" }
```

Enumerate devices:

```rust
let devices = shake::Device::enumerate()?;
for dev in devices {
    println!("{}: {}", dev.id(), dev.name());
}
```

Upload and play a rumble effect:

```rust
use shake::{Device, Effect, RumbleEffect};

let device = Device::open(0)?;

let id = device.upload(&Effect::Rumble(RumbleEffect {
    strong_magnitude: 0x4000,
    weak_magnitude: 0x2000,
    duration: 1000,
    delay: 0,
    direction: 0,
}))?;

device.play(id)?;
```

## Testing

The test suite covers:

- Mock backend behavior  
- Linux event‑node scanning  
- Effect‑to‑ff conversion logic  

Tests run without hardware:

```sh
cargo test
```

## Authors

- Giorgio (JaskRendix)  
- Artur Rojek (zear)  
- Joe Vargas (jxv)

## License

MIT License.
