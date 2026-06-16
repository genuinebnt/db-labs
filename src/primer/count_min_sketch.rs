use std::{
    cmp::min,
    collections::BinaryHeap,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    iter::zip,
    marker::PhantomData,
    sync::{
        Mutex,
        atomic::{AtomicU32, Ordering},
    },
};

use crate::common::util::hash_util;

const SEED_BASE: u64 = 15445;

#[derive(Debug)]
pub struct CountMinSketch<K> {
    width: u32,
    depth: u32,
    matrix: Vec<Vec<AtomicU32>>,
    _marker: PhantomData<K>,
}

impl<K: Hash> CountMinSketch<K> {
    pub fn new(width: u32, depth: u32) -> Self {
        assert_ne!(width, 0);
        assert_ne!(depth, 0);

        Self {
            width,
            depth,
            matrix: (0..depth)
                .map(|_| (0..width).map(|_| AtomicU32::new(0)).collect::<Vec<_>>())
                .collect(),
            _marker: PhantomData,
        }
    }

    pub fn insert(&self, item: &K) {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let h1 = hasher.finish();

        for (idx, row) in self.matrix.iter().enumerate() {
            let h2 = hash_util::combine_hashes(idx as u64, SEED_BASE);
            let h3 = hash_util::combine_hashes(h1, h2) % self.width as u64;
            row[h3 as usize].fetch_add(1, Ordering::Relaxed);
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
            min_count = min(min_count, row[h3 as usize].load(Ordering::Relaxed));
        }

        min_count
    }

    pub fn clear(&self) {
        for row in &self.matrix {
            for cell in row {
                cell.store(0, Ordering::Relaxed);
            }
        }
    }

    pub fn merge(&self, other: &CountMinSketch<K>) {
        // we want to merge self and other
        // they should have same width and depth to be compactible
        // merge should sum the values in the same place
        assert_eq!(self.width, other.width);
        assert_eq!(self.depth, other.depth);

        let self_matrix = &self.matrix;
        let other_matrix = &other.matrix;

        for (self_row, other_row) in zip(self_matrix.iter(), other_matrix.iter()) {
            for (self_col, other_col) in zip(self_row, other_row) {
                let other_val = other_col.load(Ordering::Relaxed);
                self_col.fetch_add(other_val, Ordering::Relaxed);
            }
        }
    }

    pub fn top_k(&self, k: usize, candidates: &[K]) -> Vec<(K, u32)>
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
    use std::{sync::Arc, thread::spawn};

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

    #[test]
    fn parallel_test() {
        for iter in (500..1000).step_by(100) {
            for num_threads in (8..16).step_by(2) {
                let cms: Arc<CountMinSketch<String>> = Arc::new(CountMinSketch::new(500, 15));
                let mut threads = Vec::new();
                threads.reserve(num_threads);

                for _ in 0..num_threads {
                    let cms = Arc::clone(&cms);
                    let handle = std::thread::spawn(move || {
                        for j in 0..iter {
                            cms.insert(&"frequent".to_string());
                            if j % 3 == 0 {
                                cms.insert(&"less_frequent".to_string());
                            }
                        }
                    });

                    threads.push(handle);
                }

                for thread in threads {
                    thread.join().unwrap();
                }

                let top = cms.top_k(2, &["frequent".to_owned(), "less_frequent".to_owned()]);
                assert_eq!(2, top.len());

                assert_eq!(top[0].0, "frequent".to_string());
                assert_eq!(top[0].1, (num_threads * iter) as u32);

                assert_eq!(top[1].0, "less_frequent".to_string());
                let expected_less_count_iter = (iter as f64 / 3.0).ceil() as u32;
                let expected_less_freq_count_iter = num_threads as u32 * expected_less_count_iter;
                assert_eq!(top[1].1, expected_less_freq_count_iter);
            }
        }
    }

    // Port of BusTub CountMinSketchTest.ComplexParallelTest
    // Two shared sketches filled concurrently, then clear + merge.
    // Requires clear()/merge() to take &self so they're callable through Arc.
    #[test]
    fn complex_parallel_test() {
        for iterations in (200..=500usize).step_by(100) {
            let num_threads = 8usize;
            let cms1: Arc<CountMinSketch<i32>> = Arc::new(CountMinSketch::new(1000, 10));
            let cms2: Arc<CountMinSketch<i32>> = Arc::new(CountMinSketch::new(1000, 10));

            // Group 1: threads inserting into cms1.
            let mut handles = Vec::new();
            for i in 0..num_threads as i32 {
                let cms1 = Arc::clone(&cms1);
                handles.push(spawn(move || {
                    for j in 0..iterations {
                        cms1.insert(&i);
                        cms1.insert(&42);
                        if j % 2 == 0 {
                            cms1.insert(&100);
                        }
                    }
                }));
            }
            // Group 2: half as many threads inserting into cms2.
            for i in 0..(num_threads / 2) as i32 {
                let cms2 = Arc::clone(&cms2);
                handles.push(spawn(move || {
                    for j in 0..iterations / 2 {
                        cms2.insert(&(i + 100));
                        cms2.insert(&42);
                        if j % 3 == 0 {
                            cms2.insert(&200);
                        }
                    }
                }));
            }
            for h in handles {
                h.join().unwrap();
            }

            // cms1 counts.
            for i in 0..num_threads as i32 {
                if i != 42 && i != 100 {
                    assert_eq!(cms1.count(&i), iterations as u32);
                }
            }
            assert_eq!(cms1.count(&42), (num_threads * iterations) as u32);
            assert_eq!(cms1.count(&100), (num_threads * (iterations / 2)) as u32);

            // clear() through the Arc, then everything reads 0.
            cms1.clear();
            for i in 0..iterations as i32 {
                assert_eq!(cms1.count(&i), 0);
            }
            assert_eq!(cms1.count(&42), 0);
            assert_eq!(cms1.count(&100), 0);

            // cms2 counts.
            for i in 0..(num_threads / 2) as i32 {
                assert_eq!(cms2.count(&(i + 100)), (iterations / 2) as u32);
            }
            assert_eq!(
                cms2.count(&42),
                ((num_threads / 2) * (iterations / 2)) as u32
            );
            let expected_200_iter = (iterations / 2).div_ceil(3);
            assert_eq!(
                cms2.count(&200),
                ((num_threads / 2) * expected_200_iter) as u32
            );

            // Insert a unique item then merge cms2 in.
            cms1.insert(&100);
            cms1.merge(&cms2);
            assert!(cms1.count(&42) > 0);
            assert_eq!(cms1.count(&100), (iterations / 2 + 1) as u32);
            assert!(cms2.count(&42) > 0);
        }
    }

    // Port of BusTub CountMinSketchTest.TopKComprehensiveTest
    // Four sketches filled by two thread groups, merged in stages, with TopK
    // checked at each stage.
    // Deviation from C++: dimensions are FIXED (C++ randomizes them). Fixed dims
    // make this reproducible; width 500 x depth 12 is large enough that counts
    // stay exact (the closest ranks differ by only 4: 2004 vs 2000).
    #[test]
    fn topk_comprehensive_test() {
        let (width, depth, k) = (500u32, 12u32, 5usize);
        let iterations = 1000usize;
        let nt1 = 12usize;
        let nt2 = 8usize;

        let cms1: Arc<CountMinSketch<String>> = Arc::new(CountMinSketch::new(width, depth));
        let cms2: Arc<CountMinSketch<String>> = Arc::new(CountMinSketch::new(width, depth));
        let cms3: Arc<CountMinSketch<String>> = Arc::new(CountMinSketch::new(width, depth));
        let cms4: Arc<CountMinSketch<String>> = Arc::new(CountMinSketch::new(width, depth));

        let songs1 = [
            "C.R.E.A.M.",
            "Protect Ya Neck",
            "Method Man",
            "Bring da Ruckus",
            "Da Mystery of Chessboxin'",
            "Can It Be All So Simple",
            "Wu-Tang Clan Ain't Nuthing ta F' Wit",
        ]
        .map(String::from);
        let songs2 = [
            "Triumph",
            "Gravel Pit",
            "Tearz",
            "C.R.E.A.M.",
            "Ice Cream",
            "Protect Ya Neck",
            "Method Man",
        ]
        .map(String::from);
        let all_songs = [
            "C.R.E.A.M.",
            "Protect Ya Neck",
            "Method Man",
            "Bring da Ruckus",
            "Da Mystery of Chessboxin'",
            "Can It Be All So Simple",
            "Wu-Tang Clan Ain't Nuthing ta F' Wit",
            "Triumph",
            "Gravel Pit",
            "Ice Cream",
            "Tearz",
        ]
        .map(String::from);

        let mut handles = Vec::new();
        for _ in 0..nt1 {
            let (c1, c2, c3, c4) = (
                Arc::clone(&cms1),
                Arc::clone(&cms2),
                Arc::clone(&cms3),
                Arc::clone(&cms4),
            );
            let (s1, s2) = (songs1.clone(), songs2.clone());
            handles.push(spawn(move || {
                for j in 0..iterations {
                    c1.insert(&s1[0]); // C.R.E.A.M.
                    if j % 2 == 0 {
                        c1.insert(&s1[1]); // Protect Ya Neck
                    }
                    if j % 3 == 0 {
                        c3.insert(&s1[2]); // Method Man
                    }
                    if j % 4 == 0 {
                        c3.insert(&s1[3]); // Bring da Ruckus
                    }
                    if j % 6 == 0 {
                        c2.insert(&s2[4]); // Ice Cream
                    }
                    if j % 12 == 0 {
                        c2.insert(&s2[5]); // Protect Ya Neck (songs2)
                    }
                    if j % 15 == 0 {
                        c4.insert(&s2[6]); // Method Man (songs2)
                    }
                }
            }));
        }
        for _ in 0..nt2 {
            let (c1, c2) = (Arc::clone(&cms1), Arc::clone(&cms2));
            let (s1, s2) = (songs1.clone(), songs2.clone());
            handles.push(spawn(move || {
                for j in 0..iterations {
                    c2.insert(&s2[0]); // Triumph
                    if j % 2 == 0 {
                        c2.insert(&s2[1]); // Gravel Pit
                    }
                    if j % 4 == 0 {
                        c2.insert(&s2[2]); // Tearz
                    }
                    if j % 5 == 0 {
                        c2.insert(&s2[3]); // C.R.E.A.M. (songs2)
                    }
                    if j % 5 == 0 {
                        c1.insert(&s1[4]); // Da Mystery of Chessboxin'
                    }
                    if j % 10 == 0 {
                        c1.insert(&s1[5]); // Can It Be All So Simple
                    }
                    if j % 20 == 0 {
                        c1.insert(&s1[6]); // Wu-Tang ...
                    }
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }

        cms1.merge(&cms3);
        cms2.merge(&cms4);

        let top1 = cms1.top_k(k, &songs1);
        assert_eq!(top1.len(), k);
        assert_eq!(top1[0].0, songs1[0]);
        assert_eq!(top1[1].0, songs1[1]);
        assert_eq!(top1[2].0, songs1[2]);
        assert_eq!(top1[3].0, songs1[3]);
        assert_eq!(top1[4].0, songs1[4]);
        assert_eq!(top1[0].1, (nt1 * iterations) as u32);
        assert_eq!(top1[1].1, (nt1 * (iterations / 2)) as u32);
        assert_eq!(top1[2].1, (nt1 * (iterations / 3 + 1)) as u32);
        assert_eq!(top1[3].1, (nt1 * (iterations / 4)) as u32);
        assert_eq!(top1[4].1, (nt2 * (iterations / 5)) as u32);

        let top2 = cms2.top_k(k, &songs2);
        assert_eq!(top2.len(), k);
        assert_eq!(top2[0].0, songs2[0]);
        assert_eq!(top2[1].0, songs2[1]);
        assert_eq!(top2[2].0, songs2[4]);
        assert_eq!(top2[3].0, songs2[2]);
        assert_eq!(top2[4].0, songs2[3]);
        assert_eq!(top2[0].1, (nt2 * iterations) as u32);
        assert_eq!(top2[1].1, (nt2 * (iterations / 2)) as u32);
        assert_eq!(top2[2].1, (nt1 * (iterations / 6 + 1)) as u32);
        assert_eq!(top2[3].1, (nt2 * (iterations / 4)) as u32);
        assert_eq!(top2[4].1, (nt2 * (iterations / 5)) as u32);

        cms1.merge(&cms2);
        let top_merged = cms1.top_k(k, &all_songs);
        assert_eq!(top_merged.len(), k);
        assert_eq!(top_merged[0].0, "C.R.E.A.M.");
        assert_eq!(top_merged[1].0, "Triumph");
        assert_eq!(top_merged[2].0, "Protect Ya Neck");
        assert_eq!(top_merged[3].0, "Method Man");
        assert_eq!(top_merged[4].0, "Gravel Pit");
        assert_eq!(
            top_merged[0].1,
            (nt1 * iterations + nt2 * (iterations / 5)) as u32
        );
        assert_eq!(top_merged[1].1, (nt2 * iterations) as u32);
        assert_eq!(
            top_merged[2].1,
            (nt1 * (iterations / 2) + nt1 * (iterations / 12 + 1)) as u32
        );
        assert_eq!(
            top_merged[3].1,
            (nt1 * (iterations / 3 + 1) + nt1 * (iterations / 15 + 1)) as u32
        );
        assert_eq!(top_merged[4].1, (nt2 * (iterations / 2)) as u32);
    }

    // Port of BusTub CountMinSketchTest.ContentionRatioTest
    // Compares concurrent (lock-free) insert against an externally-serialized
    // insert and asserts a speedup. Passes thanks to the per-cell
    // `AtomicU32::fetch_add` insert — a global-`Mutex` insert would serialize
    // internally and score ~1.0.
    //
    // The >1.2x bound is timing-based and machine-dependent; if it flakes under
    // load (CI, single core), loosen the threshold or `#[ignore]` it there.
    #[test]
    fn contention_ratio_test() {
        use std::time::Instant;

        let insert_iters: i64 = 10_000;
        let num_threads = 2;
        let cms: Arc<CountMinSketch<i64>> = Arc::new(CountMinSketch::new(500, 15));

        let mut time_with_mutex: Vec<u128> = Vec::new();
        let mut time_wo_mutex: Vec<u128> = Vec::new();

        for iter in 0..10 {
            let enable_mutex = iter % 2 == 0;
            let gmtx = Arc::new(Mutex::new(())); // external serialization lock

            let start = Instant::now();
            let mut handles = Vec::new();
            for _ in 0..num_threads {
                let cms = Arc::clone(&cms);
                let gmtx = Arc::clone(&gmtx);
                handles.push(spawn(move || {
                    for j in 0..insert_iters {
                        if enable_mutex {
                            let _guard = gmtx.lock().unwrap();
                            cms.insert(&(j % 10));
                        } else {
                            cms.insert(&(j % 10));
                        }
                    }
                }));
            }
            for h in handles {
                h.join().unwrap();
            }

            let elapsed = start.elapsed().as_micros();
            if enable_mutex {
                time_with_mutex.push(elapsed);
            } else {
                time_wo_mutex.push(elapsed);
            }
        }

        // Correctness: every value inserted 10 iters * 2 threads * 1000 = 20000.
        // (j % 10 over 0..10_000 hits each of 0..9 exactly 1000 times per run.)
        for i in 0..10i64 {
            assert_eq!(cms.count(&i), 20000);
        }

        let sum_wo: u128 = time_wo_mutex.iter().sum();
        let sum_with: u128 = time_with_mutex.iter().sum();
        let speedup = sum_with as f64 / sum_wo as f64;
        println!("lock-free (us):  {time_wo_mutex:?}");
        println!("serialized (us): {time_with_mutex:?}");
        println!("speedup: {speedup}");

        assert!(
            speedup > 1.2,
            "speedup {speedup} not > 1.2 — insert is not lock-free yet"
        );
    }
}
