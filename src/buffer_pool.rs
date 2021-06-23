use std::cell::{Cell, RefCell};
use std::io::Seek;
use std::ops::{Index, IndexMut};
use std::rc::Rc;

use crate::constant::PAGE_SIZE;
use crate::ids::{BufferId, PageId};

pub type Page = [u8; PAGE_SIZE];
pub struct Buffer {
    pub page_id: PageId,
    pub page: RefCell<Page>,
    pub is_dirty: Cell<bool>,
}

pub struct Frame {
    pub usage_count: u64,
    pub buffer: Rc<Buffer>,
}

pub struct BufferPool {
    buffers: Vec<Frame>,
    next_victim_id: BufferId,
}

impl BufferPool {

    fn size(&self) -> usize {
        self.buffers.len()
    }

    //Clock-sweep
    fn evict(&mut self) -> Option<BufferId> {
        let pool_size = self.size();
        let mut consecutibe_pinned = 0;
        let victim_id = loop {
            let next_victim_id = self.next_victim_id;
            let frame = &mut self[next_victim_id];
            if frame.usage_count == 0 {
                break self.next_victim_id;
            }

            if Rc::get_mut(&mut frame.buffer).is_some() {
                frame.usage_count -= 1;
                consecutibe_pinned = 0;
            } else {
                consecutibe_pinned += 1;
                if consecutibe_pinned >= pool_size {
                    return None;
                }
            }
            self.next_victim_id = self.increment_id(self.next_victim_id);
        };

        Some(victim_id)
    }

    fn increment_id(&self, buffer_id: BufferId) -> BufferId {
        BufferId((buffer_id.0 + 1) % self.size())
    }
}

impl Index<BufferId> for BufferPool {
    type Output = Frame;

    fn index(&self, index: BufferId) -> &Self::Output {
        &self.buffers[index.0]
    }
}

impl IndexMut<BufferId> for BufferPool {
    fn index_mut(&mut self, index: BufferId) -> &mut Self::Output {
        &mut self.buffers[index.0]
    }
}