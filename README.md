# libShake

A simple, cross‑platform haptic library written in Rust.

## Overview

libShake provides a unified API for force‑feedback (haptic) devices across multiple platforms.  
The goal is to offer a clean, safe, and modern interface for rumble, periodic, constant, and ramp effects without exposing platform‑specific details.

The library currently includes:

- **Linux backend** using the `evdev` force‑feedback subsystem  
- **macOS backend (stub)** — modern macOS versions no longer ship ForceFeedback.framework, so the backend compiles but reports `ShakeError::Support`  
- A shared, platform‑agnostic `Device` API  
- A rich `Effect` model supporting rumble, periodic, constant, and ramp effects, including envelopes and timing

## Installation

### Linux

libShake builds natively on Linux using Cargo:

```sh
cargo build --release
```

The Linux backend uses `nix` + `libc` to access `/dev/input/event*` devices and supports all effect types.

### macOS

Modern macOS versions (11+) no longer include `ForceFeedback.framework`.  
`build.rs` detects this and generates a stub `ffi.rs` so the crate compiles, but **force‑feedback is not available**.

The macOS backend currently returns `ShakeError::Support` for all operations.

## Usage

Add libShake to your `Cargo.toml`:

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
    duration: 1000, // milliseconds
    delay: 0,
}))?;

device.play(id)?;
```

## Authors

- Artur Rojek (zear)  
- Joe Vargas (jxv)

## License

MIT License.
