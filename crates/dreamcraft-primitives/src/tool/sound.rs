use dreamcraft_core::{DreamError, Id, Result, Tick};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AudioEffect {
    Equalizer {
        low_db: f64,
        mid_db: f64,
        high_db: f64,
    },
    Compressor {
        threshold_db: f64,
        ratio: f64,
        attack_ms: f64,
        release_ms: f64,
    },
    Reverb {
        room_size: f64, // 0.0 .. 1.0
        damping: f64,
        wet_dry: f64, // 0.0 .. 1.0
    },
    Delay {
        time_ms: f64,
        feedback: f64,
        mix: f64,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioClip {
    pub id: Id,
    pub name: String,
    pub source_path: String,
    pub start: Tick,
    pub duration: Tick,
    pub source_in: Tick,
    pub gain_db: f64,
    pub pan: f64, // -100.0 .. +100.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioTrack {
    pub id: Id,
    pub name: String,
    pub volume_db: f64, // -inf .. +12 dB
    pub pan: f64,       // -100 (left) .. +100 (right)
    pub muted: bool,
    pub solo: bool,
    pub effects: Vec<AudioEffect>,
    pub clips: Vec<AudioClip>,
}

impl AudioTrack {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            volume_db: 0.0,
            pan: 0.0,
            muted: false,
            solo: false,
            effects: Vec::new(),
            clips: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoundProject {
    pub id: Id,
    pub name: String,
    pub sample_rate: u32,
    pub bpm: f64,
    pub tracks: Vec<AudioTrack>,
    pub master_volume_db: f64,
}

impl SoundProject {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            sample_rate: 48000,
            bpm: 120.0,
            tracks: vec![
                AudioTrack::new("Vocal"),
                AudioTrack::new("Drums"),
                AudioTrack::new("Bass"),
                AudioTrack::new("Synth"),
            ],
            master_volume_db: 0.0,
        }
    }

    pub fn add_track(&mut self, name: impl Into<String>) -> Id {
        let t = AudioTrack::new(name);
        let id = t.id;
        self.tracks.push(t);
        id
    }

    pub fn insert_clip(
        &mut self,
        track_id: Id,
        name: impl Into<String>,
        source_path: impl Into<String>,
        start: Tick,
        duration: Tick,
    ) -> Result<Id> {
        let track = self
            .tracks
            .iter_mut()
            .find(|t| t.id == track_id)
            .ok_or_else(|| DreamError::General(format!("Audio track {} not found", track_id)))?;

        let clip = AudioClip {
            id: Id::new(),
            name: name.into(),
            source_path: source_path.into(),
            start,
            duration,
            source_in: Tick::ZERO,
            gain_db: 0.0,
            pan: 0.0,
        };
        let id = clip.id;
        track.clips.push(clip);
        Ok(id)
    }

    pub fn set_fader(&mut self, track_id: Id, volume_db: f64) -> Result<()> {
        let track = self
            .tracks
            .iter_mut()
            .find(|t| t.id == track_id)
            .ok_or_else(|| DreamError::General(format!("Audio track {} not found", track_id)))?;
        track.volume_db = volume_db;
        Ok(())
    }

    pub fn set_pan(&mut self, track_id: Id, pan: f64) -> Result<()> {
        let track = self
            .tracks
            .iter_mut()
            .find(|t| t.id == track_id)
            .ok_or_else(|| DreamError::General(format!("Audio track {} not found", track_id)))?;
        track.pan = pan.clamp(-100.0, 100.0);
        Ok(())
    }

    pub fn add_effect(&mut self, track_id: Id, effect: AudioEffect) -> Result<()> {
        let track = self
            .tracks
            .iter_mut()
            .find(|t| t.id == track_id)
            .ok_or_else(|| DreamError::General(format!("Audio track {} not found", track_id)))?;
        track.effects.push(effect);
        Ok(())
    }
}
