use std::{path::PathBuf, time::Duration};

use tokio::{process::Command, task::JoinHandle};

/// Used for match timers and sfx
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Match {
    /// When we started the match
    pub start: std::time::Instant,
}

impl Match {
    /// Creates a new match, starting now
    pub fn new() -> Match {
        Match {
            start: std::time::Instant::now(),
        }
    }

    /// Creates a new match from a given start time
    pub fn from_start_time(start_time: std::time::Instant) -> Match {
        Match { start: start_time }
    }

    /// Returns the current phase of the match
    pub fn phase(&self) -> MatchPhase {
        let elapsed = self.start.elapsed();

        if elapsed < MatchPhase::Autonomous.length() {
            return MatchPhase::Autonomous;
        }

        if elapsed < (MatchPhase::Autonomous.length() + MatchPhase::Transition.length()) {
            return MatchPhase::Transition;
        }

        if elapsed
            < (MatchPhase::Autonomous.length()
                + MatchPhase::Transition.length()
                + MatchPhase::Teleop.length())
        {
            return MatchPhase::Teleop;
        }

        MatchPhase::None
    }

    /// Returns how much is left before the end of this phase
    pub fn remaining_in_phase(&self) -> Duration {
        let elapsed = self.start.elapsed();

        match self.phase() {
            MatchPhase::None => Duration::from_secs(0),
            MatchPhase::Autonomous => MatchPhase::Autonomous.length() - elapsed,
            MatchPhase::Transition => {
                (MatchPhase::Autonomous.length() + MatchPhase::Transition.length()) - elapsed
            }
            MatchPhase::Teleop => {
                (MatchPhase::Autonomous.length()
                    + MatchPhase::Transition.length()
                    + MatchPhase::Teleop.length())
                    - elapsed
            }
        }
    }

    /// Returns whether the match is already over
    pub fn is_over(&self) -> bool {
        self.phase() == MatchPhase::None
    }

    /// Returns the full duration of a match
    pub fn length() -> std::time::Duration {
        MatchPhase::Autonomous.length()
            + MatchPhase::Transition.length()
            + MatchPhase::Teleop.length()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MatchPhase {
    Autonomous,
    Transition,
    Teleop,

    /// After the match has already ended
    None,
}

impl MatchPhase {
    /// Returns how many seconds the phase lasts
    pub fn length_seconds(&self) -> u64 {
        match *self {
            Self::Autonomous => 30,
            Self::Transition => 8,
            Self::Teleop => 120,
            MatchPhase::None => 0,
        }
    }

    /// Returns a strongly-typed length of the phase
    pub fn length(&self) -> Duration {
        Duration::from_secs(self.length_seconds())
    }
}

/// Plays SFX for an active match
#[derive(Debug)]
pub struct MatchSFXHandler {
    /// A receiver for updating the active match
    pub receiver: tokio::sync::broadcast::Receiver<Option<Match>>,

    /// A join handle for the current player thread
    pub thread: Option<JoinHandle<()>>,
}

impl MatchSFXHandler {
    // This is the easiest solution I've found to play sounds
    /// Construct a command on the local OS to play the written sound file
    fn construct_play_command(path: &PathBuf) -> Option<Command> {
        cfg_if::cfg_if! {
            if #[cfg(target_os = "linux")] {
                let mut command = Command::new("aplay");
                command.arg("--quiet").arg(path.to_str().expect("Play command file path was not valid unicode -- please open an issue on github: https://github.com/firstslovenia/FTCTui/issues"));
                Some(command)
            } else if #[cfg(target_os = "macos")] {
                let mut command = Command::new("afplay");
                command.arg(path.to_str().expect("Play command file path was not valid unicode -- please open an issue on github: https://github.com/firstslovenia/FTCTui/issues"));
                Some(command)
            } else if #[cfg(target_os = "windows")] {
                let mut command = Command::new("powershell");
                command.arg("-c");
                command.arg(format!("(New-Object Media.SoundPlayer \"{}\").PlaySync();", path.to_str().expect("Play command file path was not valid unicode -- please open an issue on github: https://github.com/firstslovenia/FTCTui/issues")));
                Some(command)
            } else {
                None
            }
        }
    }

    /// Plays the provided sound
    pub async fn play_sound(sfx: MatchSFX) {
        log::info!("Playing {:?}", sfx);

        let path = match sfx.ensure_on_filesystem() {
            Ok(p) => p,
            Err(e) => {
                log::error!("Failed to write temp file for audio playback: {e}");
                return;
            }
        };

        let Some(mut command) = Self::construct_play_command(&path) else {
            log::error!(
                "Failed to create audio playback command, are you on linux, macos or windows?"
            );
            return;
        };

        let status = match command.status().await {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to spawn playback command: {e}");
                return;
            }
        };

        if !status.success() {
            log::error!("Playback command returned non-zero exit status: {status}");
        }
    }

    /// A thread which plays audio by awaiting timeouts
    /// for certain elapsed intervals
    pub async fn player_thread(active_match: Match) {
        MatchSFXHandler::play_sound(MatchSFX::MatchStart01).await;

        let transition_start = active_match.start + MatchPhase::Autonomous.length();
        tokio::time::sleep_until(transition_start.into()).await;

        MatchSFXHandler::play_sound(MatchSFX::Drivers02).await;

        let countdown_start =
            transition_start + (MatchPhase::Transition.length() - Duration::from_secs(3));
        tokio::time::sleep_until(countdown_start.into()).await;

        MatchSFXHandler::play_sound(MatchSFX::Countdown03).await;

        let teleop_start = transition_start + MatchPhase::Transition.length();
        tokio::time::sleep_until(teleop_start.into()).await;

        MatchSFXHandler::play_sound(MatchSFX::TeleopStart04).await;

        let endgame_start = teleop_start + (MatchPhase::Teleop.length() - Duration::from_secs(30));
        tokio::time::sleep_until(endgame_start.into()).await;

        MatchSFXHandler::play_sound(MatchSFX::Endgame05).await;

        let end = teleop_start + MatchPhase::Teleop.length();
        tokio::time::sleep_until(end.into()).await;

        MatchSFXHandler::play_sound(MatchSFX::End06).await;
    }

    /// Handles spawning player threads and killing them if we get a new match
    pub async fn handler_thread(&mut self) {
        loop {
            match self.receiver.recv().await {
                Ok(update) => match update {
                    Some(new_match) => {
                        let thread =
                            tokio::spawn(async move { Self::player_thread(new_match).await });

                        if let Some(old_thread) = &self.thread {
                            old_thread.abort();
                        }

                        self.thread = Some(thread);
                    }
                    None => {
                        if let Some(old_thread) = &self.thread {
                            old_thread.abort();
                        }
                    }
                },
                Err(e) => {
                    log::warn!("Failed to receive new match! {:?}", e);
                }
            }
        }
    }

    /// Spawns a new SFX handler, returning the channel to send matches with
    pub async fn spawn() -> tokio::sync::broadcast::Sender<Option<Match>> {
        let (sender, receiver) = tokio::sync::broadcast::channel(2);

        let mut sfx_handler = MatchSFXHandler {
            thread: None,
            receiver,
        };

        tokio::task::spawn(async move { sfx_handler.handler_thread().await });

        sender
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MatchSFX {
    MatchStart01,
    Drivers02,
    Countdown03,
    TeleopStart04,
    Endgame05,
    End06,
    Abortmatch,
}

impl MatchSFX {
    /// Returns the included audio bytes
    pub fn wav_file(&self) -> &[u8] {
        match self {
            MatchSFX::MatchStart01 => MATCH_START_01_WAV,
            MatchSFX::Drivers02 => DRIVERS_02_WAV,
            MatchSFX::Countdown03 => COUNTDOWN_03_WAV,
            MatchSFX::TeleopStart04 => TELEOP_START_04_WAV,
            MatchSFX::Endgame05 => ENDGAME_05_WAV,
            MatchSFX::End06 => END_06_WAV,
            MatchSFX::Abortmatch => ABORT_MATCH_WAV,
        }
    }

    /// Returns a filename for this sfx (used for [MatchSFX::ensure_on_filesystem])
    pub fn file_name(&self) -> &'static str {
        match self {
            MatchSFX::MatchStart01 => "ftctui_01.wav",
            MatchSFX::Drivers02 => "ftctui_02.wav",
            MatchSFX::Countdown03 => "ftctui_03.wav",
            MatchSFX::TeleopStart04 => "ftctui_04.wav",
            MatchSFX::Endgame05 => "ftctui_05.wav",
            MatchSFX::End06 => "ftctui_06.wav",
            MatchSFX::Abortmatch => "ftctui_abort.wav",
        }
    }

    /// Ensures the sfx is on the filesystem
    pub fn ensure_on_filesystem(&self) -> std::io::Result<std::path::PathBuf> {
        let bytes = self.wav_file();

        let mut file_path = std::env::temp_dir();
        file_path.push(self.file_name());

        match std::fs::write(&file_path, bytes) {
            Ok(_) => Ok(file_path),
            Err(e) => Err(e),
        }
    }
}

pub const MATCH_START_01_WAV: &[u8; 35902] = include_bytes!("../assets/01-match-start-charge.wav");
pub const DRIVERS_02_WAV: &[u8; 103270] =
    include_bytes!("../assets/02-beep beep beep, drivers pick up your controlers.wav");
pub const COUNTDOWN_03_WAV: &[u8; 534258] = include_bytes!("../assets/03-3-2-1.wav");
pub const TELEOP_START_04_WAV: &[u8; 23451] =
    include_bytes!("../assets/04-driver controlled start-firebell.wav");
pub const ENDGAME_05_WAV: &[u8; 47300] = include_bytes!("../assets/05-engame-factwhistle.wav");
pub const END_06_WAV: &[u8; 66230] = include_bytes!("../assets/06-endmatch.wav");
pub const ABORT_MATCH_WAV: &[u8; 56698] = include_bytes!("../assets/abort_match.wav");
