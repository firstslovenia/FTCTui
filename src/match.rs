use std::time::Duration;

use tokio::task::JoinHandle;

/// Used for match timers and sfx
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Match {
    /// When we started the match
    pub start: std::time::Instant,
}

impl Match {
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
    /// Plays the provided sound
    pub async fn play_sound(sfx: MatchSFX) {
        log::info!("Play {:?}", sfx);
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
