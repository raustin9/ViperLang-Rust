use std::{
    alloc::{
        alloc, dealloc, Layout
    },
    ptr::NonNull
};

use crate::{constants, error::AllocError};

#[derive(Debug, PartialEq)]
pub enum BlockError {
    /// Bad requested block size and therefore alignment was not a power of two
    BadRequest,

    /// Not enough available memory to serve this request
    OOM,
}

pub type BlockPtr = NonNull<u8>;
pub type BlockSize = usize;

pub struct Block {
    ptr: BlockPtr,
    size: BlockSize,
}


impl Block {
    pub fn new(size: BlockSize) -> Result<Block, BlockError> {
        if !size.is_power_of_two() {
            return Err(BlockError::BadRequest);
        }

        return Ok(Block{
            ptr: alloc_block(size)?,
            size
        });
    }
    pub fn as_ptr(&self) -> *const u8 {
        return self.ptr.as_ptr();
    }
}

impl Drop for Block {
    fn drop(&mut self) {
        dealloc_block(self.ptr, self.size);
    }
}

/// Allocate a block of memory
pub fn alloc_block(size: BlockSize) -> Result<BlockPtr, BlockError> {
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, size);

        let ptr = alloc(layout);
        if ptr.is_null() {
            return Err(BlockError::OOM);
        } else {
            return Ok(NonNull::new_unchecked(ptr));
        }
    }
}

/// Deallocate a block of memory
pub fn dealloc_block(ptr: BlockPtr, size: BlockSize) {
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, size);

        dealloc(ptr.as_ptr(), layout);
    }
}


/// Object size class.
/// - Small objects fit inside a line
/// - Medium objects span more than one line
/// - Large objects span multiple blocks
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SizeClass {
    Small,
    Medium,
    Large,
}

impl SizeClass {
    pub fn get_for_size(object_size: usize) -> Result<SizeClass, AllocError> {
        match object_size {
            constants::SMALL_OBJECT_MIN..=constants::SMALL_OBJECT_MAX => {
                return Ok(SizeClass::Small);
            },
            constants::MEDIUM_OBJECT_MIN..=constants::MEDIUM_OBJECT_MAX => {
                return Ok(SizeClass::Medium);
            },
            constants::LARGE_OBJECT_MIN..=constants::LARGE_OBJECT_MAX => {
                return Ok(SizeClass::Large);
            }
            _ => {
                return Err(AllocError::BadRequest);
            }
        }
    }
}
