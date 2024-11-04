
pub const MAX_SEGMENT_SIZE: usize = 0xfffff;
pub const NO_OFFSET: usize = 0;
pub const NULL_SEGMENT: u8 = 0;
pub const KERNEL_CODE_SEGMENT: u8 = PRESENT | TYPE | EXECUTABLE | READABLE_WRITABLE;
pub const KERNEL_DATA_SEGMENT: u8 = PRESENT | TYPE | READABLE_WRITABLE;
pub const KERNEL_STACK_SEGMENT: u8 = PRESENT | TYPE | READABLE_WRITABLE | DIRECTION_CONFORMING;
pub const USER_CODE_SEGMENT: u8 = PRESENT | DPL | TYPE | READABLE_WRITABLE | EXECUTABLE;
pub const USER_DATA_SEGMENT: u8 = PRESENT | DPL | TYPE | READABLE_WRITABLE;
pub const USER_STACK_SEGMENT: u8 = PRESENT | DPL | TYPE | READABLE_WRITABLE | DIRECTION_CONFORMING;
pub const SEGMENT_FLAGS: u8 = GRANULARITY | DB_SIZE | SEGMENT_SIZE_HIGH;
const GRANULARITY: u8 = 1 << 7;
const DB_SIZE: u8 = 1 << 6;

#[allow(dead_code)]
const LONG_MODE: u8 = 1 << 5;

#[allow(dead_code)]
const RESERVED: u8 = 1 << 4;
const SEGMENT_SIZE_HIGH: u8 = (MAX_SEGMENT_SIZE >> 16) as u8;
const PRESENT: u8 = 1 << 7;
const DPL: u8 = 1 << 6 | 1 << 5;
const TYPE: u8 = 1 << 4;
const EXECUTABLE: u8 = 1 << 3;
const DIRECTION_CONFORMING: u8 = 1 << 2;
const READABLE_WRITABLE: u8 = 1 << 1;

#[allow(dead_code)]
const ACCESSED: u8 = 1 << 0;
