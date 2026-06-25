use crate::{buffer::replacer::Replacer, common::config::FrameId};
use std::collections::{HashMap, VecDeque, hash_map::Entry};

/// LRU-K replacer: evicts the frame with the largest backward k-distance (the
/// distance back to its k-th most recent access). Frames with fewer than k
/// accesses have +inf distance and are evicted first, in classic LRU order.
#[derive(Debug)]
pub struct LRUKReplacer {
    /// Capacity: maximum number of frames (used to bound valid frame ids).
    pub replacer_size: usize,
    /// The "k" in LRU-K: how many recent accesses define the backward distance.
    pub k: usize,
    /// Per-frame access history, keyed by frame id.
    pub store: HashMap<FrameId, LRUKNode>,
    /// Logical clock, bumped on every access; supplies access timestamps.
    pub current_timestamp: usize,
    /// Number of currently evictable frames (this is what `size()` reports).
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
        // Best candidate so far: (backward k-distance, oldest timestamp, frame id).
        let mut candidate: Option<(f64, usize, FrameId)> = None;

        for node in &self.store {
            let frame_id = node.0;
            let node = node.1;

            // Only evictable frames are eligible.
            if !node.is_evictable {
                continue;
            }

            // `history` is capped at k entries, so the front is the k-th most recent
            // access (or, with fewer than k accesses, the oldest recorded access).
            let oldest_distance = *&node.history.front().expect("node has no history");
            // Backward k-distance = now - (k-th most recent access). With fewer than k
            // accesses the distance is +inf, so such frames are evicted first.
            let kth_backward_distance = if node.history.len() == self.k {
                (self.current_timestamp - oldest_distance) as f64
            } else {
                f64::INFINITY
            };

            // Prefer the larger backward k-distance; break ties (notably the all-+inf
            // case) by evicting the oldest timestamp, i.e. classic LRU.
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

        // `remove` drops the frame and decrements `current_size`.
        let (_, _, frame_id) = candidate?;
        self.remove(frame_id);
        Some(frame_id)
    }

    fn record_access(&mut self, frame_id: FrameId) {
        assert!(frame_id < self.replacer_size);

        // Advance the logical clock; this access's timestamp.
        self.current_timestamp += 1;

        // Create the node on first access, then append the timestamp either way so the
        // first access is never lost. Keep only the k most recent (drop the oldest).
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

        // No-op if untracked. Removing a tracked frame must only happen when it is
        // evictable, and it drops one from the evictable count.
        if let Entry::Occupied(entry) = self.store.entry(frame_id) {
            assert!(entry.get().is_evictable);
            entry.remove();
            self.current_size -= 1;
        }
    }

    fn set_evictable(&mut self, frame_id: FrameId, set_evictable: bool) {
        assert!(frame_id < self.replacer_size);

        // No-op on untracked frames. Adjust `current_size` only on an actual transition.
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

/// Per-frame access bookkeeping for one tracked frame.
#[derive(Debug)]
pub struct LRUKNode {
    /// Timestamps of the up-to-k most recent accesses (oldest at the front).
    pub history: VecDeque<usize>,
    pub k: usize,
    pub frame_id: FrameId,
    /// Whether this frame may currently be chosen for eviction.
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
        // `evict` returns `Option<FrameId>`: `None` means nothing was evicted, `Some(id)`
        // carries the evicted frame. Assertions below compare directly against `None`/`Some(_)`.

        // Initialize the replacer.
        let mut lru_replacer = LRUKReplacer::new(7, 2);

        // Add six frames to the replacer. We now have frames [1, 2, 3, 4, 5]. We set frame 6 as non-evictable.
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

        // The size of the replacer is the number of frames that can be evicted, _not_ the total number of frames entered.
        assert_eq!(lru_replacer.size(), 5);

        // Record an access for frame 1. Now frame 1 has two accesses total.
        lru_replacer.record_access(1);
        // All other frames now share the maximum backward k-distance. Since we use timestamps to break ties, where the first
        // to be evicted is the frame with the oldest timestamp, the order of eviction should be [2, 3, 4, 5, 1].

        // Evict three pages from the replacer.
        // To break ties, we use LRU with respect to the oldest timestamp, or the least recently used frame.
        assert_eq!(lru_replacer.evict(), Some(2));
        assert_eq!(lru_replacer.evict(), Some(3));
        assert_eq!(lru_replacer.evict(), Some(4));
        assert_eq!(lru_replacer.size(), 2);
        // Now the replacer has the frames [5, 1].

        // Insert new frames [3, 4], and update the access history for 5. Now, the ordering is [3, 1, 5, 4].
        lru_replacer.record_access(3);
        lru_replacer.record_access(4);
        lru_replacer.record_access(5);
        lru_replacer.record_access(4);
        lru_replacer.set_evictable(3, true);
        lru_replacer.set_evictable(4, true);
        assert_eq!(lru_replacer.size(), 4);

        // Look for a frame to evict. We expect frame 3 to be evicted next.
        assert_eq!(lru_replacer.evict(), Some(3));
        assert_eq!(lru_replacer.size(), 3);

        // Set 6 to be evictable. 6 should be evicted next since it has the maximum backward k-distance.
        lru_replacer.set_evictable(6, true);
        assert_eq!(lru_replacer.size(), 4);
        assert_eq!(lru_replacer.evict(), Some(6));
        assert_eq!(lru_replacer.size(), 3);

        // Mark frame 1 as non-evictable. We now have [5, 4].
        lru_replacer.set_evictable(1, false);

        // We expect frame 5 to be evicted next.
        assert_eq!(lru_replacer.size(), 2);
        assert_eq!(lru_replacer.evict(), Some(5));
        assert_eq!(lru_replacer.size(), 1);

        // Update the access history for frame 1 and make it evictable. Now we have [4, 1].
        lru_replacer.record_access(1);
        lru_replacer.record_access(1);
        lru_replacer.set_evictable(1, true);
        assert_eq!(lru_replacer.size(), 2);

        // Evict the last two frames.
        assert_eq!(lru_replacer.evict(), Some(4));
        assert_eq!(lru_replacer.size(), 1);
        assert_eq!(lru_replacer.evict(), Some(1));
        assert_eq!(lru_replacer.size(), 0);

        // Insert frame 1 again and mark it as non-evictable.
        lru_replacer.record_access(1);
        lru_replacer.set_evictable(1, false);
        assert_eq!(lru_replacer.size(), 0);

        // A failed eviction should not change the size of the replacer.
        assert_eq!(lru_replacer.evict(), None);

        // Mark frame 1 as evictable again and evict it.
        lru_replacer.set_evictable(1, true);
        assert_eq!(lru_replacer.size(), 1);
        assert_eq!(lru_replacer.evict(), Some(1));
        assert_eq!(lru_replacer.size(), 0);

        // There is nothing left in the replacer, so make sure this doesn't do something strange.
        assert_eq!(lru_replacer.evict(), None);
        assert_eq!(lru_replacer.size(), 0);

        // Make sure that setting a nonexistent frame as evictable or non-evictable doesn't do something strange.
        lru_replacer.set_evictable(6, false);
        lru_replacer.set_evictable(6, true);
    }
}
