# Examples

Small programs that show how to use libShake.

## List devices
`list_devices.rs`  
Prints all detected devices and their capabilities.

## Simple rumble
`simple_rumble.rs`  
Uploads and plays a rumble effect on device 0.

## Simple periodic
`simple_periodic.rs`  
Uploads and plays a periodic effect on device 0.

## Capacity test
`capacity.rs`  
Uploads as many effects as the device reports it can hold.

## Playback test
`playback.rs`  
Plays, stops, and replays a single effect.

## Order test
`order.rs`  
Uploads several effects and plays them in random order.

## Update test
`update.rs`  
Uploads an effect, plays it, updates it, and plays it again.

## Mixing test
`mixing.rs`  
Plays multiple effects at the same time.

Run any example with:

```sh
cargo run --example <name>
```
