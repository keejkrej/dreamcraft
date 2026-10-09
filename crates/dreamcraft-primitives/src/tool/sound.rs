use dreamcraft_core::{DreamError, Id, Result, Tick};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParametricEqBand {
    pub enabled: bool,
    pub freq_hz: f64,
    pub gain_db: f64,
    pub q: f64,
}

impl Default for ParametricEqBand {
    fn default() -> Self {
        Self {
            enabled: true,
            freq_hz: 1000.0,
            gain_db: 0.0,
            q: 0.707,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParametricEq {
    pub hpf_enabled: bool,
    pub hpf_freq: f64,
    pub low_shelf: ParametricEqBand,
    pub low_mid: ParametricEqBand,
    pub high_mid: ParametricEqBand,
    pub high_shelf: ParametricEqBand,
    pub lpf_enabled: bool,
    pub lpf_freq: f64,
}

impl Default for ParametricEq {
    fn default() -> Self {
        Self {
            hpf_enabled: false,
            hpf_freq: 30.0,
            low_shelf: ParametricEqBand { enabled: true, freq_hz: 100.0, gain_db: 0.0, q: 0.707 },
            low_mid: ParametricEqBand { enabled: true, freq_hz: 500.0, gain_db: 0.0, q: 1.0 },
            high_mid: ParametricEqBand { enabled: true, freq_hz: 2500.0, gain_db: 0.0, q: 1.0 },
            high_shelf: ParametricEqBand { enabled: true, freq_hz: 8000.0, gain_db: 0.0, q: 0.707 },
            lpf_enabled: false,
            lpf_freq: 18000.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Compressor {
    pub threshold_db: f64,
    pub ratio: f64,
    pub attack_ms: f64,
    pub release_ms: f64,
    pub knee_db: f64,
    pub makeup_gain_db: f64,
}

impl Default for Compressor {
    fn default() -> Self {
        Self {
            threshold_db: -18.0,
            ratio: 4.0,
            attack_ms: 10.0,
            release_ms: 100.0,
            knee_db: 3.0,
            makeup_gain_db: 2.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reverb {
    pub room_size: f64,     // 0.0 .. 1.0
    pub decay_time_s: f64,  // seconds
    pub damping: f64,       // 0.0 .. 1.0
    pub predelay_ms: f64,   // ms
    pub wet_dry: f64,       // 0.0 (dry) .. 1.0 (wet)
}

impl Default for Reverb {
    fn default() -> Self {
        Self {
            room_size: 0.6,
            decay_time_s: 1.8,
            damping: 0.4,
            predelay_ms: 20.0,
            wet_dry: 0.25,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Delay {
    pub time_ms: f64,
    pub feedback: f64, // 0.0 .. 0.95
    pub high_cut_hz: f64,
    pub ping_pong: bool,
    pub mix: f64,      // 0.0 .. 1.0
}

impl Default for Delay {
    fn default() -> Self {
        Self {
            time_ms: 375.0,
            feedback: 0.35,
            high_cut_hz: 4500.0,
            ping_pong: true,
            mix: 0.2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AudioEffect {
    Equalizer(ParametricEq),
    Compressor(Compressor),
    Reverb(Reverb),
    Delay(Delay),
    Chorus { depth: f64, rate_hz: f64, mix: f64 },
    Limiter { ceiling_db: f64, release_ms: f64 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutomationPoint {
    pub tick: Tick,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutomationLane {
    pub parameter_name: String,
    pub points: Vec<AutomationPoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuxSend {
    pub bus_id: Id,
    pub level_db: f64,
    pub pre_fader: bool,
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
    pub fade_in: Tick,
    pub fade_out: Tick,
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
    pub sends: Vec<AuxSend>,
    pub automations: Vec<AutomationLane>,
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
            sends: Vec::new(),
            automations: Vec::new(),
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
    pub time_signature: (u8, u8),
    pub tracks: Vec<AudioTrack>,
    pub master_volume_db: f64,
    pub master_limiter: Option<AudioEffect>,
}

impl SoundProject {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            sample_rate: 48000,
            bpm: 120.0,
            time_signature: (4, 4),
            tracks: vec![
                AudioTrack::new("Vocal"),
                AudioTrack::new("Drums"),
                AudioTrack::new("Bass"),
                AudioTrack::new("Synth"),
            ],
            master_volume_db: 0.0,
            master_limiter: Some(AudioEffect::Limiter {
                ceiling_db: -0.1,
                release_ms: 50.0,
            }),
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
            fade_in: Tick::ZERO,
            fade_out: Tick::ZERO,
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

    pub fn add_automation_point(
        &mut self,
        track_id: Id,
        param_name: impl Into<String>,
        tick: Tick,
        value: f64,
    ) -> Result<()> {
        let track = self
            .tracks
            .iter_mut()
            .find(|t| t.id == track_id)
            .ok_or_else(|| DreamError::General(format!("Audio track {} not found", track_id)))?;

        let name = param_name.into();
        if let Some(lane) = track.automations.iter_mut().find(|l| l.parameter_name == name) {
            lane.points.push(AutomationPoint { tick, value });
            lane.points.sort_by_key(|p| p.tick);
        } else {
            track.automations.push(AutomationLane {
                parameter_name: name,
                points: vec![AutomationPoint { tick, value }],
            });
        }
        Ok(())
    }
}
