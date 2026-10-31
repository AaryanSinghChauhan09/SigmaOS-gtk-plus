# SigmaOS-GTK+

Rust library and two small binaries for media-player and Bluetooth state
models. `MediaPlayer` tracks a selected file, playlist, volume, and playback
state; `BluetoothManager` tracks adapter and device state.

## Current integration limits

The Cargo manifest has no GTK, Bluetooth, or mpv dependency. The binaries print
initial state and exit; they do not open windows, start playback, or communicate
with a Bluetooth adapter. `play()` changes in-memory playback state, and
Bluetooth discovery changes manager state without scanning hardware.

Keep eventual desktop integrations optional, discoverable, and compatible with
the active SigmaOS theme and accessibility contracts. Do not report media or
Bluetooth support based only on these state models.

## Development

CI runs the Rust test workflow for a specific commit; it does not establish
hardware or GUI support.
