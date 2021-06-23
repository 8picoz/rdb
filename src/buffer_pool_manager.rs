use std::collections::HashMap;

use crate::buffer_pool::BufferPool;
use crate::disk_manager::DiskManager;
use crate::ids::{BufferId, PageId};

pub struct BufferPoolManager {
    disk: DiskManager,
    pool: BufferPool,
    page_table: HashMap<PageId, BufferId>,
}