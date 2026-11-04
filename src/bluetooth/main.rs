fn main() {
    println!("SigmaOS Bluetooth Manager v{}", sigmaos_gtk_plus::VERSION);
    let mut mgr = sigmaos_gtk_plus::bluetooth::BluetoothManager::new();
    mgr.power_on();
    mgr.start_discovery().expect("failed to start discovery");
    println!("Bluetooth adapter powered on. Discovering devices...");
}
