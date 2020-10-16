use super::Tween;
use hashbrown::hash_map::{Iter, IterMut, Keys, Values};
use hashbrown::HashMap;
use std::borrow::Borrow;
use std::cmp::Eq;
use std::hash::Hash;

/// Is just a wrapper over HashMap that add the delta time to all the tweens
/// inserted and remove the tweens that already finished
pub struct TweenManager<K> {
    tweens: HashMap<K, Tween>,
}

impl<K> TweenManager<K>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            tweens: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            tweens: HashMap::with_capacity(capacity),
        }
    }

    /// Remove all tweens
    pub fn clear(&mut self) {
        self.tweens.clear();
    }

    /// Remove the finished tweens
    pub fn clean(&mut self)
    where
        K: Hash + Eq,
    {
        self.tweens.retain(|_, t| !t.did_finish());
    }

    /// Push delta time on tweens
    pub fn tick(&mut self, delta: f32) {
        self.tweens.values_mut().for_each(|t| t.tick(delta));
    }

    pub fn insert(&mut self, id: K, tween: Tween) -> Option<Tween>
    where
        K: Hash + Eq,
    {
        self.tweens.insert(id, tween)
    }

    pub fn get<Q>(&mut self, id: &Q) -> Option<&Tween>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.tweens.get(id)
    }

    pub fn get_mut<Q>(&mut self, id: &Q) -> Option<&mut Tween>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.tweens.get_mut(id)
    }

    pub fn tweens(&mut self) -> Values<'_, K, Tween> {
        self.tweens.values()
    }

    pub fn keys(&self) -> Keys<'_, K, Tween> {
        self.tweens.keys()
    }

    pub fn iter(&self) -> Iter<'_, K, Tween> {
        self.tweens.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, Tween> {
        self.tweens.iter_mut()
    }

    pub fn remove<Q>(&mut self, id: &Q) -> Option<Tween>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.tweens.remove(id)
    }

    pub fn len(&self) -> usize {
        self.tweens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tweens.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.tweens.capacity()
    }
}
