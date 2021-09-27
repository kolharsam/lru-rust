// This is an implementation of the LRU Cache Eviction Policy

// TODO: Maybe make this a lib? Or just make sure that it's more generic

use std::collections::{HashMap, VecDeque};

mod lru {
    use super::*;

    // TODO: element type should be generic
    type LinkedNode = (i32, i32);

    pub struct LRU {
        lru_capacity: usize,
        lru_map: HashMap<i32, i32>,
        lru_list: VecDeque<LinkedNode>,
    }

    pub trait LRUOperators {
        fn new(capacity: usize) -> Self;
        fn get(&mut self, key: i32) -> Option<i32>;
        fn put(&mut self, key: i32, value: i32);
        fn len(&self) -> usize;
        fn show(&self);
        fn first(&self) -> Option<&LinkedNode>;
        fn last(&self) -> Option<&LinkedNode>;
    }

    impl LRUOperators for LRU {
        fn new(capacity: usize) -> Self {
            if capacity == 0 {
                panic!("Capacity of the cache cannot be 0");
            }

            LRU {
                lru_capacity: capacity,
                lru_map: HashMap::new(),
                lru_list: VecDeque::new(),
            }
        }

        fn get(&mut self, key: i32) -> Option<i32> {
            let found_element = self.lru_map.get(&key);

            if found_element.is_none() {
                let mut new_list = self.lru_list.clone();
                let mut elem_index: usize = 0;
                let mut search_elem = (0, 0);

                for elem in new_list.iter() {
                    if elem.0 == key {
                        search_elem = *elem;
                        break;
                    }
                    elem_index += 1;
                }

                new_list.remove(elem_index);
                new_list.push_front(search_elem);
                self.lru_list = new_list;

                return Some(search_elem.1);
            }

            None
        }

        fn put(&mut self, key: i32, value: i32) {
            let mut new_list = self.lru_list.clone();
            let check_res = self.lru_map.get(&key);

            if !check_res.is_none() {
                let mut index = 0;
                for elem in self.lru_list.iter() {
                    if elem.0 == key {
                        break;
                    }
                    index += 1;
                }
                new_list.remove(index);
            }

            new_list.push_front((key, value));

            if new_list.len() > self.lru_capacity {
                let last_element = new_list.back().expect("Cache empty error");
                self.lru_map.remove(&last_element.0);
                new_list.pop_back();
            }

            self.lru_list = new_list;
            self.lru_map.insert(key, value);
        }

        fn show(&self) {
            // TODO: probably should be logging here instead of just println
            println!(
                "This is the current state of cache, with capacity({}):",
                self.lru_capacity
            );
            for (ind, elem) in self.lru_list.iter().enumerate() {
                println!("{} - {:?}", ind + 1, elem);
            }
        }

        fn len(&self) -> usize {
            self.lru_list.len()
        }

        fn first(&self) -> Option<&LinkedNode> {
            self.lru_list.front()
        }

        fn last(&self) -> Option<&LinkedNode> {
            self.lru_list.back()
        }
    }
}

/// Example using this crate
///
/// use lru::LRUOperators;
/// fn main() {
///     let mut lru = lru::LRU::new(4);
///     lru.put(1, 52);
///     lru.put(2, 35);
///     lru.get(1);
///     lru.show();
/// }

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::lru;

    #[test]
    #[should_panic]
    fn test_capacity_zero() {
        let _lru = lru::LRU::new(0);
    }

    #[test]
    fn test_get_put_fn() {
        let mut lru = lru::LRU::new(2);
        lru.put(1, 23);
        lru.put(2, 53);
        let get = lru.get(2);
        assert_eq!(get, Some(53));
    }

    #[test]
    fn test_len_fn() {
        let mut lru = lru::LRU::new(2);
        lru.put(1, 2);
        assert_eq!(lru.len(), 1);
        lru.put(2, 5);
        assert_eq!(lru.len(), 2);
        lru.put(3, 4);
        assert_eq!(lru.len(), 2);
    }

    #[test]
    fn test_order_no_overflow() {
        let mut lru = lru::LRU::new(3);
        lru.put(1, 23);
        lru.put(2, 353);
        lru.put(4, 3);

        lru.get(2);
        assert_eq!(lru.first(), Some(&(2, 353)));

        lru.get(4);
        assert_eq!(lru.first(), Some(&(4, 3)));

        let get_three = lru.get(3);
        assert_eq!(get_three, None);

        assert_eq!(lru.last(), Some(&(1, 23)));
    }

    #[test]
    fn test_order_with_overflow() {
        let mut lru = lru::LRU::new(2);
        lru.put(1, 2);
        lru.put(2, 3);

        lru.get(2);
        assert_eq!(lru.first(), Some(&(2, 3)));

        lru.put(3, 5);
        assert_eq!(lru.get(1), None);
        assert_eq!(lru.first(), Some(&(3, 5)));
        assert_eq!(lru.last(), Some(&(2, 3)));

        lru.put(5, 22);
        assert_eq!(lru.first(), Some(&(5, 22)));
        assert_eq!(lru.last(), Some(&(3, 5)));
    }

    #[test]
    fn test_put_with_duplicate_key() {
        let mut lru = lru::LRU::new(2);
        lru.put(1, 2);
        lru.put(3, 4);
        lru.put(3, 22);

        assert_eq!(lru.len(), 2);
        assert_eq!(lru.first(), Some(&(3, 22)));
        assert_eq!(lru.get(3), Some(22));
        assert_eq!(lru.last(), Some(&(1, 2)));
    }
}
