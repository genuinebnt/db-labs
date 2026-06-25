use std::{
    collections::{HashMap, VecDeque, hash_map::Entry},
    f32::INFINITY,
};

use crate::{
    buffer::replacer::Replacer,
    common::config::{FrameId, PageId},
};

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

        self.store
            .entry(frame_id)
            .and_modify(|node| {
                node.history.push_back(self.current_timestamp);
                if node.history.len() > self.k {
                    node.history.pop_front();
                }
            })
            .or_insert_with(|| LRUKNode::new(self.k, frame_id));
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
