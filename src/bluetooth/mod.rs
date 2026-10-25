//! SigmaOS Bluetooth Manager
//!
//! GTK+-based Bluetooth device manager inspired by Linux Mint Blueman.
//! Follows LEAST PRIVILEGE: requests only necessary Bluetooth permissions.

use std::collections::HashMap;

/// Bluetooth device state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceState {
    /// Device discovered but not paired
    Discovered,
    /// Pairing in progress
    Pairing,
    /// Paired and ready to connect
    Paired,
    /// Actively connected
    Connected,
    /// Disconnected (was previously connected)
    Disconnected,
}

/// Represents a discovered Bluetooth device (INFORMATION HIDING: fields private)
#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    address: String,
    name: String,
    device_class: u32,
    state: DeviceState,
    rssi: Option<i16>,
}

impl BluetoothDevice {
    /// Create a new device entry (FAIL-FAST: validate address format)
    pub fn new(address: impl Into<String>, name: impl Into<String>, class: u32) -> Result<Self, &'static str> {
        let address = address.into();
        // Basic MAC address validation (DEFENSIVE PROGRAMMING)
        if address.split(':').count() != 6 {
            return Err("Invalid Bluetooth MAC address format");
        }
        Ok(Self {
            address,
            name: name.into(),
            device_class: class,
            state: DeviceState::Discovered,
            rssi: None,
        })
    }

    /// Get device MAC address (query only — COMMAND-QUERY SEPARATION)
    pub fn address(&self) -> &str { &self.address }

    /// Get device display name (query only)
    pub fn name(&self) -> &str { &self.name }

    /// Get current state (query only)
    pub fn state(&self) -> &DeviceState { &self.state }

    /// Set RSSI signal strength reading (command only)
    pub fn update_rssi(&mut self, rssi: i16) {
        self.rssi = Some(rssi);
    }

    /// Transition device state (FAIL-FAST: invalid transitions return error)
    pub fn set_state(&mut self, new_state: DeviceState) -> Result<(), &'static str> {
        match (&self.state, &new_state) {
            (DeviceState::Discovered, DeviceState::Pairing) => {}
            (DeviceState::Pairing, DeviceState::Paired) => {}
            (DeviceState::Pairing, DeviceState::Discovered) => {}  // pairing cancelled
            (DeviceState::Paired, DeviceState::Connected) => {}
            (DeviceState::Connected, DeviceState::Disconnected) => {}
            (DeviceState::Disconnected, DeviceState::Connected) => {}
            _ => return Err("invalid state transition"),
        }
        self.state = new_state;
        Ok(())
    }
}

/// Bluetooth manager — discovers, pairs, and connects devices
/// (MODULAR: pure logic, no GTK dependency here — GTK layer is separate)
pub struct BluetoothManager {
    devices: HashMap<String, BluetoothDevice>,
    adapter_powered: bool,
    discovering: bool,
}

impl BluetoothManager {
    /// Create a new manager (adapter starts powered off — SECURE DEFAULTS)
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            adapter_powered: false,
            discovering: false,
        }
    }

    /// Power on the Bluetooth adapter
    pub fn power_on(&mut self) {
        self.adapter_powered = true;
    }

    /// Power off the adapter — also stops discovery (SAFE: cascading state)
    pub fn power_off(&mut self) {
        self.adapter_powered = false;
        self.discovering = false;
    }

    /// Start device discovery (FAIL-FAST: requires powered adapter)
    pub fn start_discovery(&mut self) -> Result<(), &'static str> {
        if !self.adapter_powered {
            return Err("Bluetooth adapter is not powered on");
        }
        self.discovering = true;
        Ok(())
    }

    /// Register a discovered device
    pub fn add_device(&mut self, device: BluetoothDevice) {
        self.devices.insert(device.address().to_string(), device);
    }

    /// Get all known devices (query only — COMMAND-QUERY SEPARATION)
    pub fn devices(&self) -> impl Iterator<Item = &BluetoothDevice> {
        self.devices.values()
    }

    /// Check if currently discovering (query only)
    pub fn is_discovering(&self) -> bool {
        self.discovering
    }
}

impl Default for BluetoothManager {
    fn default() -> Self { Self::new() }
}
