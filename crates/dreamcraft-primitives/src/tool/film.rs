use dreamcraft_core::{DreamError, Id, Result, Tick, TimeRange};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackKind {
    Video,
    Audio,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackItem {
    pub id: Id,
    pub name: String,
    pub source_path: String,
    /// Timeline start in sequence ticks.
    pub start: Tick,
    /// Duration on timeline in ticks.
    pub duration: Tick,
    /// Media source in-point in ticks.
    pub source_in: Tick,
    /// Speed multiplier (1.0 = 100%).
    pub speed: f64,
    /// Audio volume in dB or video opacity (0.0..1.0).
    pub gain: f64,
}

impl TrackItem {
    pub fn new(
        name: impl Into<String>,
        source_path: impl Into<String>,
        start: Tick,
        duration: Tick,
    ) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            source_path: source_path.into(),
            start,
            duration,
            source_in: Tick::ZERO,
            speed: 1.0,
            gain: 1.0,
        }
    }

    pub fn time_range(&self) -> TimeRange {
        TimeRange::new(self.start, self.duration)
    }

    pub fn end(&self) -> Tick {
        self.start + self.duration
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub id: Id,
    pub kind: TrackKind,
    pub name: String,
    pub items: Vec<TrackItem>,
    pub muted: bool,
    pub locked: bool,
}

impl Track {
    pub fn new(kind: TrackKind, name: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            kind,
            name: name.into(),
            items: Vec::new(),
            muted: false,
            locked: false,
        }
    }

    pub fn sort_items(&mut self) {
        self.items.sort_by_key(|item| item.start);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SequenceSettings {
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub sample_rate: u32,
}

impl Default for SequenceSettings {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            fps: 24.0,
            sample_rate: 48000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sequence {
    pub id: Id,
    pub name: String,
    pub settings: SequenceSettings,
    pub video_tracks: Vec<Track>,
    pub audio_tracks: Vec<Track>,
    pub playhead: Tick,
}

impl Sequence {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            settings: SequenceSettings::default(),
            video_tracks: vec![
                Track::new(TrackKind::Video, "V1"),
                Track::new(TrackKind::Video, "V2"),
            ],
            audio_tracks: vec![
                Track::new(TrackKind::Audio, "A1"),
                Track::new(TrackKind::Audio, "A2"),
            ],
            playhead: Tick::ZERO,
        }
    }

    pub fn add_track(&mut self, kind: TrackKind, name: impl Into<String>) -> Id {
        let track = Track::new(kind, name);
        let id = track.id;
        match kind {
            TrackKind::Video => self.video_tracks.push(track),
            TrackKind::Audio => self.audio_tracks.push(track),
        }
        id
    }

    pub fn find_track_mut(&mut self, track_id: Id) -> Option<&mut Track> {
        self.video_tracks
            .iter_mut()
            .chain(self.audio_tracks.iter_mut())
            .find(|t| t.id == track_id)
    }

    /// Append or insert a clip onto a track at a specific time.
    pub fn insert_clip(
        &mut self,
        track_id: Id,
        name: impl Into<String>,
        source_path: impl Into<String>,
        start: Tick,
        duration: Tick,
    ) -> Result<Id> {
        let track = self
            .find_track_mut(track_id)
            .ok_or_else(|| DreamError::Timeline(format!("Track not found: {}", track_id)))?;

        let item = TrackItem::new(name, source_path, start, duration);
        let clip_id = item.id;
        track.items.push(item);
        track.sort_items();
        Ok(clip_id)
    }

    /// Razor tool: cuts a clip at `cut_time` into two distinct items.
    pub fn razor_cut(&mut self, track_id: Id, cut_time: Tick) -> Result<(Id, Id)> {
        let track = self
            .find_track_mut(track_id)
            .ok_or_else(|| DreamError::Timeline(format!("Track not found: {}", track_id)))?;

        let idx = track
            .items
            .iter()
            .position(|item| item.time_range().contains(cut_time))
            .ok_or_else(|| {
                DreamError::Timeline(format!("No clip under cut time at {}", cut_time.to_seconds()))
            })?;

        let original = track.items.remove(idx);
        let offset = cut_time - original.start;

        let mut left = original.clone();
        left.duration = offset;

        let mut right = original;
        right.id = Id::new();
        right.start = cut_time;
        right.duration = right.duration - offset;
        right.source_in = right.source_in + offset;

        let left_id = left.id;
        let right_id = right.id;

        track.items.push(left);
        track.items.push(right);
        track.sort_items();

        Ok((left_id, right_id))
    }

    /// Ripple delete: removes a clip and shifts all subsequent clips earlier to fill the gap.
    pub fn ripple_delete(&mut self, clip_id: Id) -> Result<Tick> {
        let mut deleted_range: Option<TimeRange> = None;

        for track in self.video_tracks.iter_mut().chain(self.audio_tracks.iter_mut()) {
            if let Some(pos) = track.items.iter().position(|i| i.id == clip_id) {
                let removed = track.items.remove(pos);
                deleted_range = Some(removed.time_range());
                break;
            }
        }

        let range = deleted_range
            .ok_or_else(|| DreamError::Timeline(format!("Clip not found: {}", clip_id)))?;

        // Shift everything after range.end() backwards by range.duration
        let shift = range.duration;
        for track in self.video_tracks.iter_mut().chain(self.audio_tracks.iter_mut()) {
            for item in &mut track.items {
                if item.start >= range.end() {
                    item.start = item.start - shift;
                }
            }
            track.sort_items();
        }

        Ok(shift)
    }

    /// Trim clip duration (either Head trim or Tail trim).
    pub fn trim_clip(&mut self, clip_id: Id, delta_duration: Tick, trim_tail: bool) -> Result<()> {
        for track in self.video_tracks.iter_mut().chain(self.audio_tracks.iter_mut()) {
            if let Some(item) = track.items.iter_mut().find(|i| i.id == clip_id) {
                if trim_tail {
                    let new_dur = item.duration + delta_duration;
                    if new_dur.0 <= 0 {
                        return Err(DreamError::Timeline("Cannot trim duration to <= 0".into()));
                    }
                    item.duration = new_dur;
                } else {
                    // Head trim
                    let new_dur = item.duration - delta_duration;
                    if new_dur.0 <= 0 {
                        return Err(DreamError::Timeline("Cannot trim duration to <= 0".into()));
                    }
                    item.start = item.start + delta_duration;
                    item.source_in = item.source_in + delta_duration;
                    item.duration = new_dur;
                }
                track.sort_items();
                return Ok(());
            }
        }
        Err(DreamError::Timeline(format!("Clip not found: {}", clip_id)))
    }

    /// Computes total sequence duration.
    pub fn total_duration(&self) -> Tick {
        let mut max_end = Tick::ZERO;
        for track in self.video_tracks.iter().chain(self.audio_tracks.iter()) {
            for item in &track.items {
                if item.end() > max_end {
                    max_end = item.end();
                }
            }
        }
        max_end
    }

    /// Export sequence as an Edit Decision List (EDL) text format.
    pub fn export_edl(&self) -> String {
        let mut edl = format!("TITLE: {}\nFCM: NON-DROP FRAME\n\n", self.name);
        let mut event_num = 1;
        for track in &self.video_tracks {
            for item in &track.items {
                let src_in = item.source_in.format_timecode(self.settings.fps);
                let src_out = (item.source_in + item.duration).format_timecode(self.settings.fps);
                let dst_in = item.start.format_timecode(self.settings.fps);
                let dst_out = item.end().format_timecode(self.settings.fps);

                edl.push_str(&format!(
                    "{:03}  AX       V     C        {} {} {} {}\n* FROM CLIP: {}\n\n",
                    event_num, src_in, src_out, dst_in, dst_out, item.name
                ));
                event_num += 1;
            }
        }
        edl
    }
}
