
pub struct PageId(pub u64);

impl PageId {
    pub fn to_u64(&self) -> u64 {
        self.0
    }
}

pub struct BufferId(pub u64);

impl BufferId {
    pub fn to_u64(&self) -> u64 {
        self.0
    }
}