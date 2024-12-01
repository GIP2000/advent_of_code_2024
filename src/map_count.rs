use std::{cmp::Eq, collections::HashMap, hash::Hash};

pub struct Counter<V>(HashMap<V, usize>)
where
    V: Hash + Eq;

impl<V> Counter<V>
where
    V: Hash + Eq,
{
    pub fn get(&self, key: &V) -> usize {
        self.0.get(key).map(|x| *x).unwrap_or(0)
    }

    pub fn get_mut(&mut self, key: &V) -> Option<&mut usize> {
        self.0.get_mut(key)
    }

    pub fn increment(&mut self, key: V) {
        match self.get_mut(&key) {
            Some(x) => {
                *x += 1;
            }
            None => {
                self.0.insert(key, 1);
            }
        }
    }

    pub fn set(&mut self, key: V, val: usize) {
        self.0.insert(key, val);
    }
}

impl<V> From<Counter<V>> for HashMap<V, usize>
where
    V: Hash + Eq,
{
    fn from(value: Counter<V>) -> Self {
        value.0
    }
}

impl<V> From<HashMap<V, usize>> for Counter<V>
where
    V: Hash + Eq,
{
    fn from(value: HashMap<V, usize>) -> Self {
        Self(value)
    }
}

impl<V> Default for Counter<V>
where
    V: Hash + Eq,
{
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<V> Extend<V> for Counter<V>
where
    V: Hash + Eq,
{
    fn extend<T: IntoIterator<Item = V>>(&mut self, iter: T) {
        for val in iter {
            self.increment(val);
        }
    }
}

impl<V> FromIterator<V> for Counter<V>
where
    V: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut map = Counter(HashMap::new());

        for val in iter {
            map.increment(val);
        }

        return map;
    }
}
