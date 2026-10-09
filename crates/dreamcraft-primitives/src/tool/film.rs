use dreamcraft_core::{DreamError, Id, Result, Tick, TimeRange};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackKind {
    Video,
    Audio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionKind {
    CrossDissolve,
    DipToBlack,
    DipToWhite,
    WipeLeft,
    WipeRight,
    Iris,
    AudioConstantPower,
    AudioConstantGain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transition {
    pub id: Id,
    pub kind: TransitionKind,
    pub start: Tick,
    pub duration: Tick,
    pub from_clip: Option<Id>,
    pub to_clip: Option<Id>,
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
    /// Speed multiplier (1.0 = 100%, -1.0 = reverse 100%).
    pub speed: f64,
    /// Audio volume in dB or video opacity (0.0..1.0).
    pub gain: f64,
    /// Stereo pan (-1.0 to 1.0)
    pub pan: f64,
    /// Media handle limit (how much media exists before source_in and after source_out)
    pub media_duration: Option<Tick>,
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
            pan: 0.0,
            media_duration: None,
        }
    }

    pub fn time_range(&self) -> TimeRange {
        TimeRange::new(self.start, self.duration)
    }

    pub fn end(&self) -> Tick {
        self.start + self.duration
    }

    pub fn source_out(&self) -> Tick {
        self.source_in + Tick((self.duration.0 as f64 * self.speed.abs()).round() as i64)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub id: Id,
    pub kind: TrackKind,
    pub name: String,
    pub items: Vec<TrackItem>,
    pub transitions: Vec<Transition>,
    pub muted: bool,
    pub solo: bool,
    pub locked: bool,
    pub sync_locked: bool,
    pub volume_db: f64,
}

impl Track {
    pub fn new(kind: TrackKind, name: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            kind,
            name: name.into(),
            items: Vec::new(),
            transitions: Vec::new(),
            muted: false,
            solo: false,
            locked: false,
            sync_locked: true,
            volume_db: 0.0,
        }
    }

    pub fn sort_items(&mut self) {
        self.items.sort_by_key(|item| item.start);
    }

    pub fn item_by_id(&self, id: Id) -> Option<&TrackItem> {
        self.items.iter().find(|i| i.id == id)
    }

    pub fn item_by_id_mut(&mut self, id: Id) -> Option<&mut TrackItem> {
        self.items.iter_mut().find(|i| i.id == id)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SequenceSettings {
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub sample_rate: u32,
    pub drop_frame: bool,
}

impl Default for SequenceSettings {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            fps: 24.0,
            sample_rate: 48000,
            drop_frame: false,
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
                Track::new(TrackKind::Video, "V3"),
            ],
            audio_tracks: vec![
                Track::new(TrackKind::Audio, "A1"),
                Track::new(TrackKind::Audio, "A2"),
                Track::new(TrackKind::Audio, "A3"),
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

    pub fn find_track(&self, track_id: Id) -> Option<&Track> {
        self.video_tracks
            .iter()
            .chain(self.audio_tracks.iter())
            .find(|t| t.id == track_id)
    }

    pub fn find_track_mut(&mut self, track_id: Id) -> Option<&mut Track> {
        self.video_tracks
            .iter_mut()
            .chain(self.audio_tracks.iter_mut())
            .find(|t| t.id == track_id)
    }

    /// Append or place a clip onto a track at a specific time.
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

        if track.locked {
            return Err(DreamError::Timeline("Track is locked".into()));
        }

        let item = TrackItem::new(name, source_path, start, duration);
        let clip_id = item.id;
        track.items.push(item);
        track.sort_items();
        Ok(clip_id)
    }

    /// Premiere-style Overwrite: places an item replacing any existing material under its time range.
    pub fn overwrite(
        &mut self,
        track_id: Id,
        item: TrackItem,
    ) -> Result<Id> {
        let track = self
            .find_track_mut(track_id)
            .ok_or_else(|| DreamError::Timeline(format!("Track not found: {}", track_id)))?;

        if track.locked {
            return Err(DreamError::Timeline("Track is locked".into()));
        }

        let range = item.time_range();
        let clip_id = item.id;

        // Split clips overlapping range.start and range.end
        Self::clear_track_range_internal(track, range);
        track.items.push(item);
        track.sort_items();
        Ok(clip_id)
    }

    /// Premiere-style Insert edit: opens a gap at `at` across sync-locked tracks and places clip.
    pub fn insert_with_ripple(
        &mut self,
        track_id: Id,
        item: TrackItem,
    ) -> Result<Id> {
        let target_start = item.start;
        let dur = item.duration;

        // Shift everything at or after target_start by dur on all sync-locked tracks
        for track in self.video_tracks.iter_mut().chain(self.audio_tracks.iter_mut()) {
            if track.sync_locked && !track.locked {
                for i in &mut track.items {
                    if i.start >= target_start {
                        i.start = i.start + dur;
                    }
                }
                track.sort_items();
            }
        }

        let track = self
            .find_track_mut(track_id)
            .ok_or_else(|| DreamError::Timeline(format!("Track not found: {}", track_id)))?;
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

        if track.locked {
            return Err(DreamError::Timeline("Track is locked".into()));
        }

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

    /// Ripple delete: removes a clip and shifts all subsequent clips earlier to close the gap.
    pub fn ripple_delete(&mut self, clip_id: Id) -> Result<Tick> {
        let mut deleted_range: Option<TimeRange> = None;

        for track in self.video_tracks.iter_mut().chain(self.audio_tracks.iter_mut()) {
            if let Some(pos) = track.items.iter().position(|i| i.id == clip_id) {
                if track.locked {
                    return Err(DreamError::Timeline("Track is locked".into()));
                }
                let removed = track.items.remove(pos);
                deleted_range = Some(removed.time_range());
                break;
            }
        }

        let range = deleted_range
            .ok_or_else(|| DreamError::Timeline(format!("Clip not found: {}", clip_id)))?;

        let shift = range.duration;
        for track in self.video_tracks.iter_mut().chain(self.audio_tracks.iter_mut()) {
            if track.sync_locked && !track.locked {
                for item in &mut track.items {
                    if item.start >= range.end() {
                        item.start = item.start - shift;
                    }
                }
                track.sort_items();
            }
        }

        Ok(shift)
    }

    /// Premiere-style Lift: removes material within `range` leaving empty black/silence gap.
    pub fn lift(&mut self, track_id: Id, range: TimeRange) -> Result<Vec<TrackItem>> {
        let track = self
            .find_track_mut(track_id)
            .ok_or_else(|| DreamError::Timeline(format!("Track not found: {}", track_id)))?;

        if track.locked {
            return Err(DreamError::Timeline("Track is locked".into()));
        }

        let removed = Self::clear_track_range_internal(track, range);
        Ok(removed)
    }

    /// Premiere-style Extract: lifts material within `range` and ripples downstream clips backwards.
    pub fn extract(&mut self, range: TimeRange) -> Result<()> {
        let dur = range.duration;
        for track in self.video_tracks.iter_mut().chain(self.audio_tracks.iter_mut()) {
            if !track.locked {
                Self::clear_track_range_internal(track, range);
            }
        }

        // Ripple back all sync-locked tracks
        for track in self.video_tracks.iter_mut().chain(self.audio_tracks.iter_mut()) {
            if track.sync_locked && !track.locked {
                for item in &mut track.items {
                    if item.start >= range.end() {
                        item.start = item.start - dur;
                    }
                }
                track.sort_items();
            }
        }

        Ok(())
    }

    /// Roll Edit: adjust the cut boundary between two adjacent clips without changing overall sequence length.
    pub fn roll(&mut self, left_clip_id: Id, right_clip_id: Id, delta: Tick) -> Result<()> {
        let (mut left_found, mut right_found) = (false, false);

        for track in self.video_tracks.iter_mut().chain(self.audio_tracks.iter_mut()) {
            if let Some(left) = track.item_by_id_mut(left_clip_id) {
                let new_dur = left.duration + delta;
                if new_dur.0 <= 0 {
                    return Err(DreamError::Timeline("Left clip duration would be <= 0".into()));
                }
                left.duration = new_dur;
                left_found = true;
            }
            if let Some(right) = track.item_by_id_mut(right_clip_id) {
                let new_dur = right.duration - delta;
                if new_dur.0 <= 0 {
                    return Err(DreamError::Timeline("Right clip duration would be <= 0".into()));
                }
                right.start = right.start + delta;
                right.source_in = right.source_in + delta;
                right.duration = new_dur;
                right_found = true;
            }
            track.sort_items();
        }

        if left_found && right_found {
            Ok(())
        } else {
            Err(DreamError::Timeline("Could not find both adjacent clips for roll edit".into()))
        }
    }

    /// Slip Edit: adjust a clip's source in/out point without changing its placement or duration on timeline.
    pub fn slip(&mut self, clip_id: Id, delta: Tick) -> Result<()> {
        for track in self.video_tracks.iter_mut().chain(self.audio_tracks.iter_mut()) {
            if let Some(item) = track.item_by_id_mut(clip_id) {
                let new_src = item.source_in + delta;
                if new_src.0 < 0 {
                    return Err(DreamError::Timeline("Source in cannot be negative".into()));
                }
                item.source_in = new_src;
                return Ok(());
            }
        }
        Err(DreamError::Timeline(format!("Clip not found: {}", clip_id)))
    }

    /// Slide Edit: move a clip earlier/later on timeline while rolling the adjacent clips to accommodate it.
    pub fn slide(&mut self, track_id: Id, clip_id: Id, delta: Tick) -> Result<()> {
        let track = self
            .find_track_mut(track_id)
            .ok_or_else(|| DreamError::Timeline(format!("Track not found: {}", track_id)))?;

        let idx = track
            .items
            .iter()
            .position(|i| i.id == clip_id)
            .ok_or_else(|| DreamError::Timeline(format!("Clip not found: {}", clip_id)))?;

        if idx > 0 {
            let left_dur = track.items[idx - 1].duration + delta;
            if left_dur.0 <= 0 {
                return Err(DreamError::Timeline("Left clip would be <= 0".into()));
            }
            track.items[idx - 1].duration = left_dur;
        }

        if idx + 1 < track.items.len() {
            let right = &mut track.items[idx + 1];
            let right_dur = right.duration - delta;
            if right_dur.0 <= 0 {
                return Err(DreamError::Timeline("Right clip would be <= 0".into()));
            }
            right.start = right.start + delta;
            right.source_in = right.source_in + delta;
            right.duration = right_dur;
        }

        track.items[idx].start = track.items[idx].start + delta;
        track.sort_items();
        Ok(())
    }

    /// Rate stretch / Speed change (e.g. 0.5x, 2.0x, -1.0x reverse).
    pub fn set_speed(&mut self, clip_id: Id, speed: f64, ripple: bool) -> Result<()> {
        if speed.abs() < 0.01 {
            return Err(DreamError::Timeline("Speed multiplier must be non-zero".into()));
        }

        for track in self.video_tracks.iter_mut().chain(self.audio_tracks.iter_mut()) {
            if let Some(pos) = track.items.iter().position(|i| i.id == clip_id) {
                let item = &mut track.items[pos];
                let old_dur = item.duration;
                let new_dur = Tick((item.duration.0 as f64 / speed.abs()).round() as i64);
                item.speed = speed;
                item.duration = new_dur;

                if ripple {
                    let delta = new_dur - old_dur;
                    for next in track.items.iter_mut().skip(pos + 1) {
                        next.start = next.start + delta;
                    }
                }
                track.sort_items();
                return Ok(());
            }
        }
        Err(DreamError::Timeline(format!("Clip not found: {}", clip_id)))
    }

    /// Add a transition between clips or at an edit point.
    pub fn add_transition(
        &mut self,
        track_id: Id,
        kind: TransitionKind,
        start: Tick,
        duration: Tick,
        from_clip: Option<Id>,
        to_clip: Option<Id>,
    ) -> Result<Id> {
        let track = self
            .find_track_mut(track_id)
            .ok_or_else(|| DreamError::Timeline(format!("Track not found: {}", track_id)))?;

        let tr = Transition {
            id: Id::new(),
            kind,
            start,
            duration,
            from_clip,
            to_clip,
        };
        let id = tr.id;
        track.transitions.push(tr);
        Ok(id)
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

    fn clear_track_range_internal(track: &mut Track, range: TimeRange) -> Vec<TrackItem> {
        if range.is_empty() {
            return Vec::new();
        }

        let mut new_items = Vec::new();
        let mut removed = Vec::new();

        for item in track.items.drain(..) {
            if item.end() <= range.start || item.start >= range.end() {
                new_items.push(item);
            } else if item.start < range.start && item.end() > range.end() {
                // Splits in the middle into left and right pieces
                let left_dur = range.start - item.start;
                let mut left = item.clone();
                left.duration = left_dur;

                let right_offset = range.end() - item.start;
                let mut right = item.clone();
                right.id = Id::new();
                right.start = range.end();
                right.duration = item.duration - right_offset;
                right.source_in = item.source_in + right_offset;

                new_items.push(left);
                new_items.push(right);
                removed.push(item);
            } else if item.start < range.start {
                // Trim tail
                let mut trimmed = item.clone();
                trimmed.duration = range.start - item.start;
                new_items.push(trimmed);
                removed.push(item);
            } else if item.end() > range.end() {
                // Trim head
                let offset = range.end() - item.start;
                let mut trimmed = item.clone();
                trimmed.start = range.end();
                trimmed.duration = item.duration - offset;
                trimmed.source_in = item.source_in + offset;
                new_items.push(trimmed);
                removed.push(item);
            } else {
                // Entirely enclosed
                removed.push(item);
            }
        }

        track.items = new_items;
        track.sort_items();
        removed
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_film_razor_and_ripple_delete() {
        let mut seq = Sequence::new("Test Timeline");
        let v1 = seq.video_tracks[0].id;

        // Clip from 0s to 10s
        let clip_id = seq.insert_clip(v1, "Clip A", "clip_a.mp4", Tick::ZERO, Tick::from_seconds(10.0)).unwrap();

        // Razor cut at 4s -> [0s..4s] and [4s..10s]
        let (left_id, right_id) = seq.razor_cut(v1, Tick::from_seconds(4.0)).unwrap();
        assert_eq!(left_id, clip_id);

        let v1_track = seq.find_track(v1).unwrap();
        assert_eq!(v1_track.items.len(), 2);
        assert_eq!(v1_track.items[0].duration, Tick::from_seconds(4.0));
        assert_eq!(v1_track.items[1].duration, Tick::from_seconds(6.0));

        // Ripple delete right clip -> closes gap, total duration becomes 4s
        let shift = seq.ripple_delete(right_id).unwrap();
        assert_eq!(shift, Tick::from_seconds(6.0));
        assert_eq!(seq.total_duration(), Tick::from_seconds(4.0));
    }

    #[test]
    fn test_film_roll_and_slip() {
        let mut seq = Sequence::new("Trims Timeline");
        let v1 = seq.video_tracks[0].id;

        let left = seq.insert_clip(v1, "Shot 1", "shot1.mp4", Tick::ZERO, Tick::from_seconds(5.0)).unwrap();
        let right = seq.insert_clip(v1, "Shot 2", "shot2.mp4", Tick::from_seconds(5.0), Tick::from_seconds(5.0)).unwrap();

        // Roll cut by +1.0s (left expands to 6s, right shrinks to 4s starting at 6s)
        seq.roll(left, right, Tick::from_seconds(1.0)).unwrap();

        let track = seq.find_track(v1).unwrap();
        assert_eq!(track.items[0].duration, Tick::from_seconds(6.0));
        assert_eq!(track.items[1].start, Tick::from_seconds(6.0));
        assert_eq!(track.items[1].duration, Tick::from_seconds(4.0));
        assert_eq!(seq.total_duration(), Tick::from_seconds(10.0));

        // Slip right clip by +2s (timeline position unchanged, source_in shifts by +2s)
        seq.slip(right, Tick::from_seconds(2.0)).unwrap();
        let track = seq.find_track(v1).unwrap();
        assert_eq!(track.items[1].source_in, Tick::from_seconds(3.0)); // was 1.0 from roll + 2.0
    }

    #[test]
    fn test_film_set_speed_and_transitions() {
        let mut seq = Sequence::new("Speed Timeline");
        let v1 = seq.video_tracks[0].id;
        let c1 = seq.insert_clip(v1, "Shot", "shot.mp4", Tick::ZERO, Tick::from_seconds(10.0)).unwrap();

        // Double speed (2.0x) with ripple -> duration becomes 5s
        seq.set_speed(c1, 2.0, true).unwrap();
        assert_eq!(seq.total_duration(), Tick::from_seconds(5.0));

        // Add cross dissolve transition
        let tr_id = seq.add_transition(v1, TransitionKind::CrossDissolve, Tick::from_seconds(4.0), Tick::from_seconds(1.0), Some(c1), None).unwrap();
        let track = seq.find_track(v1).unwrap();
        assert_eq!(track.transitions.len(), 1);
        assert_eq!(track.transitions[0].id, tr_id);
    }
}
