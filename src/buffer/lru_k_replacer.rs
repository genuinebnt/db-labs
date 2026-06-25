use std::collections::{HashMap, VecDeque, hash_map::Entry};

use crate::{buffer::replacer::Replacer, common::config::FrameId};

#[derive(Debug)]
pub struct LRUKReplacer {
    pub replacer_size: usize,
    pub k: usize,
    pub store: HashMap<FrameId, LRUKNode>,
    pub current_timestamp: usize,
    pub current_size: usize,
}

impl LRUKReplacer {
    pub fn new(num_pages: usize, k: usize) -> Self {
        Self {
            replacer_size: num_pages,
            k,
            store: HashMap::new(),
            current_timestamp: 0,
            current_size: 0,
        }
    }
}

impl Replacer for LRUKReplacer {
    fn size(&self) -> usize {
        self.current_size
    }

    fn evict(&mut self) -> Option<FrameId> {
        let mut candidate: Option<(f64, usize, FrameId)> = None;

        for node in &self.store {
            let frame_id = node.0;
            let node = node.1;

            if !node.is_evictable {
                continue;
            }

            let oldest_distance = *&node.history.front().expect("node has no history");
            let kth_backward_distance = if node.history.len() == self.k {
                (self.current_timestamp - oldest_distance) as f64
            } else {
                f64::INFINITY
            };

            let better = match candidate {
                None => true,
                Some((candidate_kth_distance, candidate_oldest_distance, _candidate_frame_id)) => {
                    kth_backward_distance > candidate_kth_distance
                        || (kth_backward_distance == candidate_kth_distance
                            && oldest_distance < &candidate_oldest_distance)
                }
            };

            if better {
                candidate = Some((kth_backward_distance, *oldest_distance, *frame_id));
            }
        }

        let (_, _, frame_id) = candidate?;
        self.remove(frame_id);
        Some(frame_id)
    }

    fn record_access(&mut self, frame_id: FrameId) {
        assert!(frame_id < self.replacer_size);

        self.current_timestamp += 1;

        let entry = self
            .store
            .entry(frame_id)
            .or_insert_with(|| LRUKNode::new(self.k, frame_id));

        entry.history.push_back(self.current_timestamp);
        if entry.history.len() > self.k {
            entry.history.pop_front();
        }
    }

    fn remove(&mut self, frame_id: FrameId) {
        assert!(frame_id < self.replacer_size);

        if let Entry::Occupied(entry) = self.store.entry(frame_id) {
            assert!(entry.get().is_evictable);
            entry.remove();
            self.current_size -= 1;
        }
    }

    fn set_evictable(&mut self, frame_id: FrameId, set_evictable: bool) {
        assert!(frame_id < self.replacer_size);

        self.store.entry(frame_id).and_modify(|node| {
            match (set_evictable, node.is_evictable) {
                (false, false) => {}
                (true, false) => self.current_size += 1,
                (false, true) => self.current_size -= 1,
                (true, true) => {}
            };

            node.is_evictable = set_evictable
        });
    }
}

#[derive(Debug)]
pub struct LRUKNode {
    pub history: VecDeque<usize>,
    pub k: usize,
    pub frame_id: FrameId,
    pub is_evictable: bool,
}

impl LRUKNode {
    pub fn new(k: usize, frame_id: FrameId) -> Self {
        Self {
            history: VecDeque::new(),
            k,
            frame_id,
            is_evictable: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::replacer::Replacer;

    // Ported from BusTub test/buffer/lru_k_replacer_test.cpp (DISABLED_SampleTest).
    #[test]
    fn sample_test() {
        let mut lru_replacer = LRUKReplacer::new(7, 2);

        // Add six frames [1..=6]; mark 1-5 evictable, 6 non-evictable.
        lru_replacer.record_access(1);
        lru_replacer.record_access(2);
        lru_replacer.record_access(3);
        lru_replacer.record_access(4);
        lru_replacer.record_access(5);
        lru_replacer.record_access(6);
        lru_replacer.set_evictable(1, true);
        lru_replacer.set_evictable(2, true);
        lru_replacer.set_evictable(3, true);
        lru_replacer.set_evictable(4, true);
        lru_replacer.set_evictable(5, true);
        lru_replacer.set_evictable(6, false);

        // Size = number of evictable frames, not total tracked.
        assert_eq!(lru_replacer.size(), 5);

        // Second access for frame 1; the rest share max (+inf) backward k-distance,
        // so they evict in oldest-timestamp order: [2, 3, 4, 5, 1].
        lru_replacer.record_access(1);

        assert_eq!(lru_replacer.evict(), Some(2));
        assert_eq!(lru_replacer.evict(), Some(3));
        assert_eq!(lru_replacer.evict(), Some(4));
        assert_eq!(lru_replacer.size(), 2);
        // Remaining: [5, 1].

        // Insert [3, 4], update 5; ordering becomes [3, 1, 5, 4].
        lru_replacer.record_access(3);
        lru_replacer.record_access(4);
        lru_replacer.record_access(5);
        lru_replacer.record_access(4);
        lru_replacer.set_evictable(3, true);
        lru_replacer.set_evictable(4, true);
        assert_eq!(lru_replacer.size(), 4);

        // 3 has +inf distance (single access) -> evicted next.
        assert_eq!(lru_replacer.evict(), Some(3));
        assert_eq!(lru_replacer.size(), 3);

        // 6 becomes evictable; +inf distance, oldest timestamp -> evicted next.
        lru_replacer.set_evictable(6, true);
        assert_eq!(lru_replacer.size(), 4);
        assert_eq!(lru_replacer.evict(), Some(6));
        assert_eq!(lru_replacer.size(), 3);

        // Mark 1 non-evictable -> [5, 4]; 5 has the larger backward k-distance.
        lru_replacer.set_evictable(1, false);
        assert_eq!(lru_replacer.size(), 2);
        assert_eq!(lru_replacer.evict(), Some(5));
        assert_eq!(lru_replacer.size(), 1);

        // Refresh 1 and make it evictable -> [4, 1].
        lru_replacer.record_access(1);
        lru_replacer.record_access(1);
        lru_replacer.set_evictable(1, true);
        assert_eq!(lru_replacer.size(), 2);

        // Evict the last two.
        assert_eq!(lru_replacer.evict(), Some(4));
        assert_eq!(lru_replacer.size(), 1);
        assert_eq!(lru_replacer.evict(), Some(1));
        assert_eq!(lru_replacer.size(), 0);

        // Re-insert 1, non-evictable. Failed eviction must not change size.
        lru_replacer.record_access(1);
        lru_replacer.set_evictable(1, false);
        assert_eq!(lru_replacer.size(), 0);
        assert_eq!(lru_replacer.evict(), None);

        // Make 1 evictable again and evict it.
        lru_replacer.set_evictable(1, true);
        assert_eq!(lru_replacer.size(), 1);
        assert_eq!(lru_replacer.evict(), Some(1));
        assert_eq!(lru_replacer.size(), 0);

        // Empty replacer: evict is a no-op.
        assert_eq!(lru_replacer.evict(), None);
        assert_eq!(lru_replacer.size(), 0);

        // Setting evictability on a nonexistent frame must not panic or change state.
        lru_replacer.set_evictable(6, false);
        lru_replacer.set_evictable(6, true);
    }
}
