use parking_lot::RwLock;

#[derive(Copy, Clone, Debug)]
pub(crate) enum ResourceId {
    Source(u64),
    Sound(u64),
}

#[derive(Debug, Default)]
pub(crate) struct ResourceTracker {
    pub(crate) dropped: RwLock<Vec<ResourceId>>,
}

impl ResourceTracker {
    pub fn push(&self, id: ResourceId) {
        self.dropped.write().push(id);
    }

    pub fn clean(&self) {
        self.dropped.write().clear();
    }
}
