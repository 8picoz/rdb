use std::usize;


pub struct PageId(pub u64);

impl PageId {
    pub fn to_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BufferId(pub usize);

impl BufferId {
    pub fn to_u64(self) -> u64 {
        self.0 as u64
    }
}