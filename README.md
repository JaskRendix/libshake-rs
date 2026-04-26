# libShake

A cross‑platform haptic library written in Rust.

## Overview

libShake provides a unified API for force‑feedback devices.  
It exposes rumble, periodic, constant, ramp, and condition effects (spring, friction, damper, inertia) through a backend‑agnostic interface.

The library includes:

- **Linux backend** using the evdev force‑feedback subsystem  
- **Mock backend** for development and testing  
- A shared `Device` API for enumeration, capability checks, upload, play, stop, erase, gain, and autocenter  
- An `Effect` model with envelopes, timing, direction, and condition parameters  
- A `simple` module with helpers for constructing common effects

The mock backend provides:

- Timeline tracking  
- Rumble mixing with gain  
- ASCII visualizers for periodic, ramp, and all condition effects  
- Gain and autocenter state  
- A profiler  
- Log export

## Origins

The original libShake was written in C and targeted Linux force‑feedback.  
This Rust version is a clean rewrite with a revised API, backend split, and extended effect model.

Original project: [https://github.com/zear/libShake](https://github.com/zear/libShake)

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
use shake::device::Device;

let devices = Device::enumerate()?;
for info in devices {
    println!("{}: {}", info.id(), info.name());
}
```

Upload and play a rumble effect:

```rust
use shake::device::Device;
use shake::effect::{Effect, RumbleEffect};

let dev = Device::open(0)?;

let id = dev.upload(&Effect::Rumble(RumbleEffect {
    strong_magnitude: 0x4000,
    weak_magnitude: 0x2000,
    duration: 1000,
    delay: 0,
    direction: 0,
}))?;

dev.play(id)?;
```

## Testing

The test suite covers:

- Mock backend behavior  
- Linux event‑node scanning  
- Effect‑to‑ff conversion  
- ConditionEffect conversion  
- Simple API helpers  
- Timeline, profiler, and log export in the mock backend  

Tests run without hardware:

```sh
cargo test
```

## Examples

See the `examples/` directory for:

- Basic rumble and periodic usage  
- Capacity, playback, order, update, and mixing tests  
- Condition effects on real hardware  
- Mock backend visualizers  
- Combined effect demonstrations  
- A full demo suite cycling through all effect types  

Run any example with:

```sh
cargo run --example <name>
```

## Authors

- Giorgio (JaskRendix)  
- Artur Rojek (zear)  
- Joe Vargas (jxv)

## License

MIT License.
