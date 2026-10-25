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
