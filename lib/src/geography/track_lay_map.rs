use std::collections::HashMap;

/// Represents the possible track lays of a public company
#[derive(Clone, Debug)]
pub struct TrackLayMap {
    track_lays: HashMap<(usize, usize), Vec<(i32, u32)>>,
}

impl TrackLayMap {
    pub(crate) fn new() -> Self {
        TrackLayMap {
            track_lays: HashMap::new(),
        }
    }

    pub(crate) fn insert(&mut self, key: (usize, usize), value: (i32, u32)) {
        self.track_lays
            .entry(key)
            .and_modify(|v| v.push(value))
            .or_insert_with(|| vec![value]);
    }
}
