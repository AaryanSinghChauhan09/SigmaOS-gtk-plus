//! SigmaOS Media Player module
//!
//! GTK+ media player frontend inspired by Linux Mint Celluloid (MPV wrapper).
//! Follows SEPARATION OF MECHANISM & POLICY:
//! - Mechanism: MPV subprocess control
//! - Policy: UI decisions, playlist management

use std::path::PathBuf;

/// Supported media formats (OPEN STANDARD AND INTEROPERABILITY)
pub const SUPPORTED_VIDEO: &[&str] = &["mp4", "mkv", "avi", "webm", "mov", "ogv"];
pub const SUPPORTED_AUDIO: &[&str] = &["mp3", "flac", "ogg", "wav", "aac", "opus"];

/// Player playback state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaybackState {
    /// No media loaded
    Idle,
    /// Media loaded, ready to play
    Ready,
    /// Currently playing
    Playing,
    /// Paused
    Paused,
    /// Playback ended
    Ended,
}

/// Media player — wraps MPV subprocess with safe interface
pub struct MediaPlayer {
    state: PlaybackState,
    current_file: Option<PathBuf>,
    playlist: Vec<PathBuf>,
    volume: u8,  // 0-100
    muted: bool,
}

impl MediaPlayer {
    /// Create a new player at default volume 70 (SECURE DEFAULTS)
    pub fn new() -> Self {
        Self {
            state: PlaybackState::Idle,
            current_file: None,
            playlist: Vec::new(),
            volume: 70,
            muted: false,
        }
    }

    /// Load a media file (FAIL-FAST: validate extension)
    pub fn load(&mut self, path: PathBuf) -> Result<(), &'static str> {
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        if !SUPPORTED_VIDEO.contains(&ext) && !SUPPORTED_AUDIO.contains(&ext) {
            return Err("unsupported media format");
        }
        self.current_file = Some(path);
        self.state = PlaybackState::Ready;
        Ok(())
    }

    /// Start or resume playback
    pub fn play(&mut self) -> Result<(), &'static str> {
        match &self.state {
            PlaybackState::Ready | PlaybackState::Paused => {
                self.state = PlaybackState::Playing;
                Ok(())
            }
            _ => Err("cannot play in current state"),
        }
    }

    /// Pause playback (FAIL-FAST: only valid when playing)
    pub fn pause(&mut self) -> Result<(), &'static str> {
        if self.state == PlaybackState::Playing {
            self.state = PlaybackState::Paused;
            Ok(())
        } else {
            Err("not currently playing")
        }
    }

    /// Set volume 0-100 (clamped — DEFENSIVE PROGRAMMING)
    pub fn set_volume(&mut self, vol: u8) {
        self.volume = vol.min(100);
    }

    /// Toggle mute state
    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }

    /// Add file to playlist (ZERO-ONE-INFINITY: supports unlimited tracks)
    pub fn add_to_playlist(&mut self, path: PathBuf) {
        self.playlist.push(path);
    }

    /// Query current state (COMMAND-QUERY SEPARATION)
    pub fn state(&self) -> &PlaybackState { &self.state }
    /// Query current volume
    pub fn volume(&self) -> u8 { self.volume }
    /// Query mute state
    pub fn is_muted(&self) -> bool { self.muted }
}

impl Default for MediaPlayer {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_player_creation_and_defaults() {
        // Arrange & Act
        let player = MediaPlayer::new();

        // Assert
        assert_eq!(player.state(), &PlaybackState::Idle);
        assert_eq!(player.volume(), 70);
        assert!(!player.is_muted());
    }

    #[test]
    fn test_load_supported_and_unsupported_formats() {
        // Arrange
        let mut player = MediaPlayer::new();

        // Act & Assert: Supported video
        assert!(player.load(PathBuf::from("/media/video.mp4")).is_ok());
        assert_eq!(player.state(), &PlaybackState::Ready);

        // Act & Assert: Supported audio
        assert!(player.load(PathBuf::from("/media/song.flac")).is_ok());

        // Act & Assert: Unsupported format (fails fast)
        assert!(player.load(PathBuf::from("/media/doc.pdf")).is_err());
    }

    #[test]
    fn test_playback_lifecycle() {
        // Arrange
        let mut player = MediaPlayer::new();
        player.load(PathBuf::from("/media/track.mp3")).unwrap();

        // Act: Play
        assert!(player.play().is_ok());
        assert_eq!(player.state(), &PlaybackState::Playing);

        // Act: Pause
        assert!(player.pause().is_ok());
        assert_eq!(player.state(), &PlaybackState::Paused);

        // Act: Resume play
        assert!(player.play().is_ok());
        assert_eq!(player.state(), &PlaybackState::Playing);
    }

    #[test]
    fn test_volume_clamping_and_muting() {
        // Arrange
        let mut player = MediaPlayer::new();

        // Act & Assert: Clamping at 100
        player.set_volume(150);
        assert_eq!(player.volume(), 100);

        player.set_volume(45);
        assert_eq!(player.volume(), 45);

        // Act & Assert: Mute toggling
        player.toggle_mute();
        assert!(player.is_muted());
        player.toggle_mute();
        assert!(!player.is_muted());
    }

    #[test]
    fn test_playlist_management() {
        // Arrange
        let mut player = MediaPlayer::new();

        // Act
        player.add_to_playlist(PathBuf::from("track1.ogg"));
        player.add_to_playlist(PathBuf::from("track2.opus"));

        // Assert
        assert_eq!(player.playlist.len(), 2);
    }
}
