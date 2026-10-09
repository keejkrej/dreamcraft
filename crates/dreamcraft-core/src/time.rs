use serde::{Deserialize, Serialize};

/// 254,016,000,000 ticks per second (compatible with Premiere Pro & FilmCraft).
/// Divisible cleanly by 24, 25, 30, 48, 50, 60, 100, 120, 240, 44100, 48000, 96000.
pub const TICKS_PER_SECOND: i64 = 254_016_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct Tick(pub i64);

impl Tick {
    pub const ZERO: Self = Self(0);

    pub fn from_seconds(secs: f64) -> Self {
        Self((secs * TICKS_PER_SECOND as f64).round() as i64)
    }

    pub fn to_seconds(&self) -> f64 {
        self.0 as f64 / TICKS_PER_SECOND as f64
    }

    pub fn from_frames(frames: i64, fps: f64) -> Self {
        let secs = frames as f64 / fps;
        Self::from_seconds(secs)
    }

    pub fn to_frames(&self, fps: f64) -> i64 {
        (self.to_seconds() * fps).round() as i64
    }

    pub fn format_timecode(&self, fps: f64) -> String {
        let total_frames = self.to_frames(fps).max(0);
        let fps_i = fps.round() as i64;
        let ff = total_frames % fps_i;
        let total_secs = total_frames / fps_i;
        let ss = total_secs % 60;
        let total_mins = total_secs / 60;
        let mm = total_mins % 60;
        let hh = total_mins / 60;
        format!("{:02}:{:02}:{:02}:{:02}", hh, mm, ss, ff)
    }
}

impl std::ops::Add for Tick {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Tick {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: Tick,
    pub duration: Tick,
}

impl TimeRange {
    pub fn new(start: Tick, duration: Tick) -> Self {
        Self { start, duration }
    }

    pub fn end(&self) -> Tick {
        self.start + self.duration
    }

    pub fn contains(&self, t: Tick) -> bool {
        t >= self.start && t < self.end()
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end() && other.start < self.end()
    }
}
