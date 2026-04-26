# **libShake**

A cross‑platform, backend‑agnostic haptic library written in Rust.

libShake provides a unified API for force‑feedback devices, supporting rumble, periodic, constant, ramp, and condition effects (spring, friction, damper, inertia).  
It is designed for real hardware, simulation, testing, and visualization.

---

## **Features**

### **Backend‑agnostic architecture**
libShake is built around a `Backend` trait that abstracts force‑feedback implementations.  
Two backends are included:

- **Linux backend**  
  Uses the evdev force‑feedback subsystem (`/dev/input/event*`).

- **Mock backend**  
  A fully simulated device for development, testing, visualization, and CI.

### **Unified Device API**
The `Device` type provides:

- Enumeration with stable IDs  
- Capability reporting (`DeviceCapabilities`)  
- Upload, update, play, stop, erase  
- Gain and autocenter control  
- Stable effect handles (RAII erase)  
- Path‑based and ID‑based opening  
- Backend‑independent behavior

### **Rich Effect Model**
libShake exposes:

- **RumbleEffect**  
- **PeriodicEffect** (sine, triangle, square, sawtooth)  
- **ConstantEffect**  
- **RampEffect**  
- **ConditionEffect** (spring, friction, damper, inertia)

All effects support:

- Envelopes (attack/fade)  
- Duration and delay  
- Direction (0–360°)  
- Signed/unsigned scaling  
- Backend‑safe clamping

### **Simple API**
The `simple` module provides ergonomic constructors:

- `simple_rumble()`  
- `simple_periodic()`  
- `simple_constant()`  
- `simple_ramp()`  
- `simple_spring()`, `simple_friction()`, `simple_damper()`, `simple_inertia()`

These helpers handle scaling, envelopes, durations, and direction.

### **Mock Backend Tools**
The mock backend includes:

- Timeline tracking  
- Rumble mixing with gain  
- ASCII visualizers for periodic, ramp, and condition effects  
- Gain/autocenter state  
- Effect profiler  
- Log export  
- Deterministic behavior for CI

---

## **Installation**

Add to your `Cargo.toml`:

```toml
[dependencies]
shake = { path = "path/to/libshake" }
```

### **Linux backend**
Enabled by default. Uses `nix` + `libc` to access evdev.

```sh
cargo build --release
```

---

## **Usage**

### **Enumerate devices**

```rust
use shake::device::Device;

let devices = Device::enumerate()?;
for info in devices {
    println!("{}: {}", info.id, info.name);
}
```

### **Open a device and play a rumble**

```rust
use shake::device::Device;
use shake::effect::{Effect, RumbleEffect};

let dev = Device::open(0)?;

let handle = dev.upload(&Effect::Rumble(RumbleEffect {
    strong_magnitude: 0x4000,
    weak_magnitude: 0x2000,
    duration: 1000,
    delay: 0,
    direction: 0,
}))?;

handle.play()?;
```

### **Using the simple API**

```rust
use shake::device::Device;
use shake::simple::simple_rumble;

let dev = Device::open(0)?;
let handle = dev.upload(&simple_rumble(1.0, 0.5, 0.3))?;
handle.play()?;
```

---

## **Backend Architecture**

libShake separates hardware access from the public API through the `Backend` trait:

```rust
pub trait Backend {
    type Handle;

    fn scan() -> ShakeResult<Vec<PathBuf>>;
    fn open(path: &Path) -> ShakeResult<Self::Handle>;
    fn close(handle: Self::Handle);

    fn query(handle: &Self::Handle) -> ShakeResult<RawDeviceInfo>;
    fn capabilities(handle: &Self::Handle) -> ShakeResult<DeviceCapabilities>;

    fn upload(handle: &Self::Handle, effect: &Effect) -> ShakeResult<i32>;
    fn update(handle: &Self::Handle, id: i32, effect: &Effect) -> ShakeResult<()>;
    fn play(handle: &Self::Handle, id: i32) -> ShakeResult<()>;
    fn stop(handle: &Self::Handle, id: i32) -> ShakeResult<()>;
    fn erase(handle: &Self::Handle, id: i32) -> ShakeResult<()>;

    fn set_gain(handle: &Self::Handle, value: u16) -> ShakeResult<()>;
    fn set_autocenter(handle: &Self::Handle, value: u16) -> ShakeResult<()>;
}
```

### **Metadata flow**

- **RawDeviceInfo** — backend‑observed fields (name, capacity, feature bits)  
- **DeviceCapabilities** — backend‑agnostic capability model  
- **DeviceInfo** — normalized public metadata (stable ID, max_effects, raw_features, path)  
- **Device** — wraps backend handle + cached capabilities

Backends included:

- `linux` — real hardware  
- `mock` — simulation, testing, visualization  

---

## **Testing**

The test suite covers:

- Backend trait contract  
- Mock backend behavior  
- Linux event‑node scanning  
- Effect‑to‑ff conversion  
- ConditionEffect conversion  
- Simple API helpers  
- Device enumeration, opening, and effect lifecycle  
- Capability reporting  

All tests run without hardware:

```sh
cargo test
```

The mock backend ensures deterministic CI behavior.

---

## **Examples**

The `examples/` directory includes:

- Rumble and periodic basics  
- Max‑effects, playback, order, update, and mixing tests  
- Condition effects on real hardware  
- Mock backend visualizers  
- Combined effect demos  
- A full demo cycling through all effect types  

Run any example:

```sh
cargo run --example <name>
```

---

## **Origins**

This Rust rewrite is based on the original C library:

[https://github.com/zear/libShake](https://github.com/zear/libShake)

The Rust version introduces:

- Backend abstraction  
- Stronger type system  
- Safer effect model  
- Mock backend  
- Expanded test suite  
- Cleaner API  

---

## **Authors**

- Giorgio (JaskRendix)  
- Artur Rojek (zear)  
- Joe Vargas (jxv)

---

## **License**

MIT License.
