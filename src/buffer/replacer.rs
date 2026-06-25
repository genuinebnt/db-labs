use crate::common::config::{FrameId, PageId};

pub trait Replacer {
    fn size(&self) -> usize;
    fn set_evictable(&mut self, frame_id: FrameId, set_evictable: bool);
    fn record_access(&mut self, frame_id: FrameId);
    fn evict(&mut self) -> Option<FrameId>;
    fn remove(&mut self, frame_id: FrameId);
}
