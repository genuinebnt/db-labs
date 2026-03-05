// TODO: 1. Define your `FrameId` alias (e.g., `pub type FrameId = u32;`)
// TODO: 2. Define the `Replacer` trait with methods: `victim`, `pin`, `unpin`, `size`
// TODO: 3. Define the `LRUReplacer` struct and implement the `Replacer` trait for it

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_replacer_basic() {
        let mut lru_replacer = LRUReplacer::new(7);

        lru_replacer.unpin(1);
        lru_replacer.unpin(2);
        lru_replacer.unpin(3);
        lru_replacer.unpin(4);
        lru_replacer.unpin(5);
        lru_replacer.unpin(6);

        lru_replacer.unpin(1);

        assert_eq!(
            lru_replacer.size(),
            6,
            "Size should be 6 after unpinning 6 unique frames"
        );

        assert_eq!(lru_replacer.victim(), Some(1), "Victim should be 1");
        assert_eq!(lru_replacer.victim(), Some(2), "Victim should be 2");
        assert_eq!(lru_replacer.victim(), Some(3), "Victim should be 3");

        lru_replacer.pin(3);
        lru_replacer.pin(4);

        assert_eq!(
            lru_replacer.size(),
            2,
            "Size should be 2 after evicting 3 and pinning 1"
        );

        assert_eq!(lru_replacer.victim(), Some(5), "Victim should be 5");
        assert_eq!(lru_replacer.victim(), Some(6), "Victim should be 6");

        assert_eq!(lru_replacer.size(), 0, "Size should be 0");
        assert_eq!(lru_replacer.victim(), None, "No victims should remain");
    }

    #[test]
    fn test_lru_replacer_pin_unpin_cycle() {
        let mut lru_replacer = LRUReplacer::new(3);

        lru_replacer.unpin(1);
        lru_replacer.unpin(2);
        lru_replacer.unpin(3);

        lru_replacer.pin(1);
        lru_replacer.pin(2);
        lru_replacer.pin(3);

        assert_eq!(lru_replacer.size(), 0);
        assert_eq!(lru_replacer.victim(), None);

        lru_replacer.unpin(3);
        lru_replacer.unpin(2);
        lru_replacer.unpin(1);

        assert_eq!(lru_replacer.size(), 3);

        assert_eq!(lru_replacer.victim(), Some(3));
        assert_eq!(lru_replacer.victim(), Some(2));
        assert_eq!(lru_replacer.victim(), Some(1));
    }
}
