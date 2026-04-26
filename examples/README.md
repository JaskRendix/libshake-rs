# Examples

Small programs that show how to use **libShake**.  
Run any example with:

```sh
cargo run --example <name>
```

---

## Example Index

| Example | Description |
|--------|-------------|
| **list_devices.rs** | Prints all detected devices and their capabilities. |
| **simple_rumble.rs** | Uploads and plays a rumble effect on device 0. |
| **simple_periodic.rs** | Uploads and plays a periodic effect on device 0. |
| **capacity.rs** | Uploads as many effects as the device reports it can hold. |
| **playback.rs** | Plays, stops, and replays a single effect. |
| **order.rs** | Uploads several effects and plays them in random order. |
| **update.rs** | Uploads an effect, plays it, updates it, and plays it again. |
| **mixing.rs** | Plays multiple effects at the same time. |
| **conditions.rs** | Demonstrates Spring, Friction, Damper, and Inertia on real hardware. |
| **visualize.rs** | Uses the mock backend to print ASCII visualizations for all effect types. |
| **combined_periodic_friction.rs** | Plays a periodic waveform, then activates a friction field. |
| **combined_rumble_spring.rs** | Directional rumble followed by a centering spring. |
| **combined.rs** | Manual periodic visualization + Spring ConditionEffect playback. |
| **demo_suite.rs** | Cycles through **all** effect types (Rumble, Periodic, Constant, Ramp, Spring, Friction, Damper, Inertia). |
