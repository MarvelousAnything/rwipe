use std::time::SystemTime;

#[repr(C)]
#[derive(Debug)]
pub struct FileStat {
    device_id: u64,           // ID of device containing file
    inode_number: u64,        // inode number
    mode: u32,                // protection
    hard_links: u32,          // number of hard links
    user_id: u32,             // user ID of owner
    group_id: u32,            // group ID of owner
    device_type: Option<u64>, // device ID (if special file)
    size: u64,                // total size, in bytes
    block_size: u32,          // blocksize for file system I/O
    blocks_allocated: u64,    // number of blocks allocated
    accessed: SystemTime,     // time of last access
    modified: SystemTime,     // time of last modification
    created: SystemTime,      // time of creation or last status change
}
