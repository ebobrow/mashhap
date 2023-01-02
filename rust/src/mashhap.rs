// TODO: tweak
const MAX_LOAD: f64 = 0.75;

pub struct MashHap<T> {
    // TODO: also try without `Vec`, using slice with `count` and `capacity`
    entries: Vec<Entry<T>>,

    // TODO: is it better to have this or calculate
    // `entries.iter().filter(Option::is_some).count()`?
    count: usize,
}

// TODO: instead of clone just write silly macro that recursively adds to vec?
#[derive(Clone, Debug)]
enum Entry<T> {
    Some((String, T)),
    Null,
    Tombstone,
}

impl<T> MashHap<T>
where
    T: Clone + std::fmt::Debug,
{
    pub fn new() -> Self {
        MashHap::with_capacity(8)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: vec![Entry::Null; capacity],
            count: 0,
        }
    }

    /// Sets key, overwriting previous entry if there was one. Returns true if previous entry
    /// did not exist.
    pub fn set(&mut self, key: impl ToString, value: T) -> bool {
        let key = key.to_string();
        // TODO: do we resize here or wait until we see if we're replacing a tombstone, in which
        // case we don't need to resize but the insert takes longer because it has to find the
        // index twice?
        if (self.count + 1) as f64 > self.entries.len() as f64 * MAX_LOAD {
            self.resize();
        }
        let (i, old_val) = self.find_entry(&key);
        let new_key = !matches!(old_val, Entry::Some(_));
        if new_key {
            self.count += 1;
        }
        self.entries[i] = Entry::Some((key, value));
        new_key
    }

    pub fn get(&mut self, key: impl ToString) -> Option<&T> {
        let key = key.to_string();
        match self.find_entry(&key).1 {
            Entry::Some((_, v)) => Some(v),
            _ => None,
        }
    }

    /// Deletes entry at `key`. Returns true if entry existed.
    pub fn delete(&mut self, key: impl ToString) -> bool {
        let key = key.to_string();
        match self.find_entry(&key) {
            (i, Entry::Some(_)) => {
                self.entries[i] = Entry::Tombstone;
                true
            }
            _ => false,
        }
    }

    fn find_entry(&self, key: &str) -> (usize, &Entry<T>) {
        let index = hash(key) as usize % self.entries.len();
        for i in 0..self.entries.len() {
            let entry = &self.entries[index + i];
            match entry {
                Entry::Some((k, _)) => {
                    // TODO: intern?
                    if k == key {
                        return (index + i, entry);
                    }
                }
                _ => return (index + i, entry),
            }
        }
        unreachable!()
    }

    fn resize(&mut self) {
        self.count = 0;
        let old_capacity = self.entries.len();
        let new_capacity = if old_capacity < 8 {
            8
        } else {
            old_capacity * 2
        };
        // let old_entries = std::mem::replace(&mut self.entries, Vec::with_capacity(new_capacity));
        let old_entries = std::mem::replace(&mut self.entries, vec![Entry::Null; new_capacity]);
        for entry in old_entries {
            if let Entry::Some((k, v)) = entry {
                let (i, _) = self.find_entry(&k);
                self.entries[i] = Entry::Some((k, v));
                self.count += 1;
            }
        }
    }
}

/// FNV-1a
fn hash(src: &str) -> u32 {
    src.chars().fold(2166136261, |acc, c| {
        (acc ^ (c as u32)).wrapping_mul(16777619)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut map = MashHap::new();
        assert_eq!(map.get("nonexistant"), None);
        assert_eq!(map.set("A", 10), true);
        assert_eq!(map.get("A"), Some(&10));
        assert_eq!(map.set("A", 11), false);
        assert_eq!(map.get("A"), Some(&11));
        assert_eq!(map.delete("A"), true);
        assert_eq!(map.delete("A"), false);
    }

    #[test]
    fn resizing() {
        let mut map = MashHap::with_capacity(1);
        assert_eq!(map.entries.len(), 1);
        map.set("A", 1);
        assert_eq!(map.entries.len(), 8);
        map.set("B", 1);
        assert_eq!(map.entries.len(), 8);
        map.set("C", 1);
        assert_eq!(map.entries.len(), 8);
        map.set("D", 1);
        assert_eq!(map.entries.len(), 8);
        map.set("E", 1);
        assert_eq!(map.entries.len(), 8);
        map.set("F", 1);
        assert_eq!(map.entries.len(), 8);
        map.set("G", 1);
        assert_eq!(map.entries.len(), 16);
    }
}
