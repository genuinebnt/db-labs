use std::{
    cmp::min,
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    iter::zip,
    marker::PhantomData,
};

use crate::common::util::hash_util;

const SEED_BASE: u64 = 15445;

#[derive(Debug, Clone)]
pub struct CountMinSketch<K> {
    width: u32,
    depth: u32,
    matrix: Vec<Vec<u32>>,
    _marker: PhantomData<K>,
}

impl<K: Hash> CountMinSketch<K> {
    pub fn new(width: u32, depth: u32) -> Self {
        assert_ne!(width, 0);
        assert_ne!(depth, 0);

        Self {
            width,
            depth,
            matrix: vec![vec![0; width as usize]; depth as usize],
            _marker: PhantomData,
        }
    }

    pub fn insert(&mut self, item: &K) {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let h1 = hasher.finish();

        for (idx, row) in self.matrix.iter_mut().enumerate() {
            let h2 = hash_util::combine_hashes(idx as u64, SEED_BASE);
            let h3 = hash_util::combine_hashes(h1, h2) % self.width as u64;
            row[h3 as usize] += 1;
        }
    }

    pub fn count(&self, item: &K) -> u32 {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let h1 = hasher.finish();

        let mut min_count = u32::MAX;
        for (idx, row) in self.matrix.iter().enumerate() {
            let h2 = hash_util::combine_hashes(idx as u64, SEED_BASE);
            let h3 = hash_util::combine_hashes(h1, h2) % self.width as u64;
            min_count = min(min_count, row[h3 as usize]);
        }

        min_count
    }

    pub fn clear(&mut self) {
        self.matrix.iter_mut().for_each(|row| row.fill(0));
    }

    pub fn merge(&mut self, other: &CountMinSketch<K>) {
        // we want to merge self and other
        // they should have same width and depth to be compactible
        // merge should sum the values in the same place
        assert_eq!(self.width, other.width);
        assert_eq!(self.depth, other.depth);

        for (self_row, other_row) in zip(&mut self.matrix, &other.matrix) {
            for (self_col, other_col) in zip(self_row, other_row) {
                *self_col += other_col
            }
        }
    }

    pub fn top_k(&self, mut k: usize, candidates: &[K]) -> Vec<(K, u32)>
    where
        K: Clone,
    {
        let mut heap = BinaryHeap::new();
        for candidate in candidates {
            let item = ItemWithCount::new(candidate, self.count(candidate));
            heap.push(item);
        }

        std::iter::from_fn(|| heap.pop())
            .take(k)
            .map(|iwc| (iwc.item.clone(), iwc.count))
            .collect()
    }
}

#[derive(Debug)]
struct ItemWithCount<K> {
    item: K,
    count: u32,
}

impl<K> Eq for ItemWithCount<K> {}

impl<K> PartialEq for ItemWithCount<K> {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

impl<K> Ord for ItemWithCount<K> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}

impl<K> PartialOrd for ItemWithCount<K> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<K> ItemWithCount<K> {
    fn new(item: K, count: u32) -> Self {
        Self { item, count }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Port of BusTub CountMinSketchTest.BasicTest1
    // Strings, exact counts at large width (200x12) so collisions don't inflate.
    #[test]
    fn basic_test_1() {
        let mut cms = CountMinSketch::<String>::new(200, 12);

        assert_eq!(cms.count(&"test".to_string()), 0);

        cms.insert(&"Welcome to CMU DB (15-445/645)".to_string());
        assert_eq!(cms.count(&"Welcome to CMU DB (15-445/645)".to_string()), 1);

        let names = [
            "DJ-Cache", "Sirui", "Andy", "Melody", "William", "Saransh", "Song", "Ruiqi", "David",
        ];
        for i in 0..10u32 {
            for name in names {
                cms.insert(&name.to_string());
            }
            let curr = i + 1;
            for name in names {
                assert_eq!(cms.count(&name.to_string()), curr);
            }
        }

        for name in names {
            assert_eq!(cms.count(&name.to_string()), 10);
        }
        assert_eq!(cms.count(&"Welcome to CMU DB (15-445/645)".to_string()), 1);
        assert_eq!(cms.count(&"NonExistent".to_string()), 0);
    }

    // Port of BusTub CountMinSketchTest.BasicTest2
    // i64 keys including negatives, exact counts at width 500x20.
    #[test]
    fn basic_test_2() {
        let mut cms = CountMinSketch::<i64>::new(500, 20);

        assert_eq!(cms.count(&0), 0);
        cms.insert(&0);
        assert_eq!(cms.count(&0), 1);

        for i in 0..30u32 {
            for j in 0..4u32 {
                for k in [10i64, 122, 200, 911, 15445] {
                    cms.insert(&k);
                }
                let curr = i * 4 + j + 1;
                for k in [10i64, 122, 200, 911, 15445] {
                    assert_eq!(cms.count(&k), curr);
                }
            }
            for j in 0..5u32 {
                for k in [-1i64, -2, -3, -15445] {
                    cms.insert(&k);
                }
                let curr = i * 5 + j + 1;
                for k in [-1i64, -2, -3, -15445] {
                    assert_eq!(cms.count(&k), curr);
                }
            }
        }

        assert_eq!(cms.count(&0), 1);
        for k in [10i64, 122, 200, 911, 15445] {
            assert_eq!(cms.count(&k), 120);
        }
        for k in [-1i64, -2, -3, -15445] {
            assert_eq!(cms.count(&k), 150);
        }
        assert_eq!(cms.count(&999999), 0);
    }

    // Port of BusTub CountMinSketchTest.EdgeTest2
    // Degenerate dimensions: every item collides, so counts are deterministic
    // regardless of the hash function.
    #[test]
    fn edge_test_2_min_dimensions() {
        // width = 1: everything hashes to the same column.
        let mut cms_min_width = CountMinSketch::<i64>::new(1, 20);
        cms_min_width.insert(&1);
        cms_min_width.insert(&2);
        assert_eq!(cms_min_width.count(&1), 2);
        assert_eq!(cms_min_width.count(&2), 2);
        cms_min_width.insert(&3);
        cms_min_width.insert(&4);
        for k in 1..=4i64 {
            assert_eq!(cms_min_width.count(&k), 4);
        }

        // depth = 1: no collision mitigation, count is a lower bound (>=).
        let mut cms_min_depth = CountMinSketch::<i64>::new(50, 1);
        cms_min_depth.insert(&15445);
        cms_min_depth.insert(&(15445 + 4));
        cms_min_depth.insert(&15445);
        assert!(cms_min_depth.count(&15445) >= 2);
        assert!(cms_min_depth.count(&(15445 + 4)) >= 1);

        // width = depth = 1: single bucket, every key reads the total.
        let mut cms_min_both = CountMinSketch::<i64>::new(1, 1);
        for i in 0..5i64 {
            cms_min_both.insert(&i);
        }
        for i in 0..5i64 {
            assert_eq!(cms_min_both.count(&i), 5);
        }
        assert_eq!(cms_min_both.count(&999), 5);
        assert_eq!(cms_min_both.count(&-1), 5);
        assert_eq!(cms_min_both.count(&15445), 5);
    }

    // Port of the count-preserving half of BusTub CountMinSketchTest.MoveTest.
    // Rust move (`let b = a`) is a memcpy and the moved-from binding is gone at
    // compile time, so we only assert the moved-to value retains the data.
    #[test]
    fn move_test() {
        let mut cms1 = CountMinSketch::<i32>::new(100, 10);
        for _ in 0..5 {
            cms1.insert(&1);
        }
        for _ in 0..3 {
            cms1.insert(&2);
        }
        cms1.insert(&3);
        assert_eq!(cms1.count(&1), 5);
        assert_eq!(cms1.count(&2), 3);
        assert_eq!(cms1.count(&3), 1);

        let cms2 = cms1; // move
        assert_eq!(cms2.count(&1), 5);
        assert_eq!(cms2.count(&2), 3);
        assert_eq!(cms2.count(&3), 1);
    }

    // Port of BusTub CountMinSketchTest.EdgeTest1
    // Invalid construction must be rejected. Contract chosen: new() panics on a
    // zero dimension (the Rust analogue of C++'s std::invalid_argument throw).
    #[test]
    fn edge_test_1_invalid_construction() {
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {})); // silence the expected panics
        for i in (10..50).step_by(10) {
            let zero_width = std::panic::catch_unwind(|| CountMinSketch::<i32>::new(0, i));
            let zero_depth = std::panic::catch_unwind(|| CountMinSketch::<i32>::new(i * 5, 0));
            assert!(zero_width.is_err(), "new(0, {i}) should be rejected");
            assert!(zero_depth.is_err(), "new({}, 0) should be rejected", i * 5);
        }
        std::panic::set_hook(prev);
    }

    // Port of BusTub CountMinSketchTest.ClearTest
    // clear() must zero the counters IN PLACE (keep width x depth), not drop the
    // rows. `self.matrix.clear()` empties the matrix and is wrong — this drives it.
    #[test]
    fn clear_test() {
        let mut cms = CountMinSketch::<i32>::new(200, 10);
        for _ in 0..15 {
            cms.insert(&1);
        }
        for _ in 0..10 {
            cms.insert(&2);
        }
        for _ in 0..8 {
            cms.insert(&3);
        }
        assert_eq!(cms.count(&1), 15);
        assert_eq!(cms.count(&2), 10);
        assert_eq!(cms.count(&3), 8);

        cms.clear();

        assert_eq!(cms.count(&1), 0);
        assert_eq!(cms.count(&2), 0);
        assert_eq!(cms.count(&3), 0);
        assert_eq!(cms.count(&999), 0);
    }

    // Port of BusTub CountMinSketchTest.MergeTest
    // merge() takes &other (C++ keeps `other` usable afterwards). Both sketches
    // must share width AND depth so cells line up for addition.
    #[test]
    fn merge_test() {
        // Test 1: normal merge, no collisions (250 x 8).
        let mut cms1 = CountMinSketch::<String>::new(250, 8);
        let mut cms2 = CountMinSketch::<String>::new(250, 8);

        for _ in 0..5 {
            cms1.insert(&"055".to_string());
        }
        for _ in 0..2 {
            cms1.insert(&"4987".to_string());
        }
        for _ in 0..3 {
            cms1.insert(&"3125".to_string());
        }
        cms1.insert(&"2256".to_string());

        assert_eq!(cms1.count(&"055".to_string()), 5);
        assert_eq!(cms1.count(&"4987".to_string()), 2);
        assert_eq!(cms1.count(&"3125".to_string()), 3);
        assert_eq!(cms1.count(&"2256".to_string()), 1);

        for _ in 0..5 {
            cms2.insert(&"4739".to_string());
        }
        for _ in 0..2 {
            cms2.insert(&"3125".to_string());
        }
        for _ in 0..3 {
            cms2.insert(&"4987".to_string());
        }
        for _ in 0..4 {
            cms2.insert(&"2256".to_string());
        }

        cms1.merge(&cms2);

        assert_eq!(cms1.count(&"055".to_string()), 5);
        assert_eq!(cms1.count(&"4987".to_string()), 5);
        assert_eq!(cms1.count(&"4739".to_string()), 5);
        assert_eq!(cms1.count(&"2256".to_string()), 5);
        assert_eq!(cms1.count(&"3125".to_string()), 5);

        // cms2 must remain usable — proves merge borrows rather than consumes.
        assert_eq!(cms2.count(&"4739".to_string()), 5);
        assert_eq!(cms2.count(&"3125".to_string()), 2);
        assert_eq!(cms2.count(&"4987".to_string()), 3);

        // Test 2: width = 1 collision merge.
        let mut cms3 = CountMinSketch::<i32>::new(1, 20);
        let mut cms4 = CountMinSketch::<i32>::new(1, 20);
        cms3.insert(&1);
        cms3.insert(&2);
        cms3.insert(&5);
        cms4.insert(&3);
        cms4.insert(&4);
        assert_eq!(cms3.count(&1), 3);
        assert_eq!(cms4.count(&3), 2);

        cms3.merge(&cms4);

        assert_eq!(cms3.count(&1), 5);
        assert_eq!(cms3.count(&2), 5);
        assert_eq!(cms3.count(&3), 5);
        assert_eq!(cms3.count(&4), 5);
        assert_eq!(cms3.count(&996), 5);
    }

    // Port of BusTub CountMinSketchTest.TopKBasicTest
    // top_k(k, candidates) -> Vec<(K, u32)>, ranked by estimated count desc.
    // NOTE: width is small (10) here, so a hash collision could inflate a count
    // and break the `.1` (count) assertions. If this fails by a small amount,
    // suspect a collision under DefaultHasher, not a logic bug.
    #[test]
    fn topk_basic_test() {
        for iter in 1..10u32 {
            let mut cms = CountMinSketch::<String>::new(10, 3);
            for _ in 0..(iter + 4) {
                cms.insert(&"frequent".to_string());
            }
            for _ in 0..(iter + 2) {
                cms.insert(&"medium".to_string());
            }
            for _ in 0..iter {
                cms.insert(&"rare".to_string());
            }

            let candidates = [
                "frequent".to_string(),
                "medium".to_string(),
                "rare".to_string(),
            ];
            let top = cms.top_k(3, &candidates);
            assert_eq!(top.len(), 3);
            assert_eq!(top[0].0, "frequent");
            assert_eq!(top[0].1, iter + 4);
            assert_eq!(top[1].0, "medium");
            assert_eq!(top[1].1, iter + 2);
            assert_eq!(top[2].0, "rare");
            assert_eq!(top[2].1, iter);
        }
    }

    // Port of BusTub CountMinSketchTest.TopKDynamicTest
    // Counts accumulate across cases (one sketch); only the ranking (keys) is
    // checked. width 200 keeps counts exact for these four items.
    #[test]
    fn topk_dynamic_test() {
        let test_cases = [
            [1, 2, 3, 4],
            [7, 5, 3, 1],
            [2, 2, 5, 7],
            [6, 6, 2, 2],
            [1, 3, 6, 6],
            [400, 200, 300, 100],
        ];
        let expected_orders = [
            [4, 3, 2],
            [1, 2, 3],
            [4, 3, 1],
            [1, 2, 4],
            [4, 3, 2],
            [1, 3, 2],
        ];

        let mut cms = CountMinSketch::<i32>::new(200, 15);
        for (case, expected) in test_cases.iter().zip(expected_orders.iter()) {
            for item in 1..=4i32 {
                for _ in 0..case[(item - 1) as usize] {
                    cms.insert(&item);
                }
            }
            let top = cms.top_k(3, &[1, 2, 3, 4]);
            assert_eq!(top.len(), 3);
            for rank in 0..3 {
                assert_eq!(top[rank].0, expected[rank], "case {case:?}, rank {rank}");
            }
        }
    }
}
