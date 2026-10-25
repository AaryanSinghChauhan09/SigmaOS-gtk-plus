fn main() {
    println!("SigmaOS Media Player v{}", sigmaos_gtk_plus::VERSION);
    let mut player = sigmaos_gtk_plus::player::MediaPlayer::new();
    println!("Player initialized. Volume: {}%", player.volume());
    // GTK main loop would be initialized here
}
