use std::hash::{DefaultHasher, Hash, Hasher};

pub struct BloomFilter {
    filter: Vec<bool>,
    num_hashes: u32,
}

impl BloomFilter {
    fn new(m: usize, k: u32) -> Self {
        return BloomFilter {
            filter: vec![false; m],
            num_hashes: k,
        };
    }

    // This shouldn't require Clone but I don't want to deal with lifetimes right now
    fn add<T>(&mut self, value: &T)
    where
        T: Hash + From<u32> + std::ops::Add<Output = T> + Clone,
    {
        let mut h = DefaultHasher::new();
        for hash_num in 0u32..self.num_hashes {
            let v: T = value.clone() + hash_num.into();
            v.hash(&mut h);
            let idx = h.finish() % self.num_hashes as u64;
            *self.filter.get_mut(idx as usize).unwrap() = true;
        }
    }

    fn may_contain<T>(&self, value: &T) -> bool
    where
        T: Hash + From<u32> + std::ops::Add<Output = T> + Clone,
    {
        let mut h = DefaultHasher::new();
        for hash_num in 0u32..self.num_hashes {
            let v: T = value.clone() + hash_num.into();
            v.hash(&mut h);
            let idx = h.finish() % self.num_hashes as u64;
            if !self.filter.get(idx as usize).unwrap() {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloomfilter() {
        let mut bloom_filter = BloomFilter::new(10, 3);
        bloom_filter.add(&15u32);
        assert_eq!(bloom_filter.may_contain(&15u32), true);
    }
}
