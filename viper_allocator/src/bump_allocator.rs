use crate::{
    block_meta::*, constants::{self, *}, error::*, memory::*
};
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ptr::write;
use std::mem::replace;

use crate::memory::*;

pub struct BumpBlock {
    cursor: *const u8, /// pointer to where the last object was written
    limit: *const u8, 
    block: Block,      /// the block itself being written
    meta: BlockMeta,
}

impl BumpBlock {
    /// Create a new block of heap space and it's metadata, placing a
    /// pointer to the metadata in the first word of the block.
    pub fn new() -> Result<BumpBlock, AllocError> {
        let inner_block = Block::new(constants::BLOCK_SIZE).unwrap();
        let block_ptr = inner_block.as_ptr();

        let block = BumpBlock {
            cursor: unsafe { block_ptr.add(constants::BLOCK_CAPACITY) },
            limit: block_ptr,
            block: inner_block,
            meta: BlockMeta::new(block_ptr),
        };

        Ok(block)
    }
    
    pub fn inner_alloc(&mut self, alloc_size: usize) -> Option<*const u8> {
        let block_start_ptr = self.block.as_ptr() as usize;
        let limit = self.limit as usize;

        // align to word boundary
        let align_mask = !(size_of::<usize>() - 1);

        let next_ptr = block_start_ptr.checked_sub(alloc_size)? & align_mask;

        if next_ptr < limit {
            let block_relative_limit = 
                unsafe { self.limit.sub(self.block.as_ptr() as usize) } as usize;
            
            if block_relative_limit > 0 {
                if let Some((cursor, limit)) = self.meta.find_next_available_hole(block_relative_limit, alloc_size)
                {
                    self.cursor = unsafe { self.block.as_ptr().add(cursor) };
                    self.limit = unsafe { self.block.as_ptr().add(limit) };
                    return self.inner_alloc(alloc_size);
                }
            }
            return None;
        } else {
            self.cursor = next_ptr as *const u8;
            return Some(self.cursor);
        }

    }
    
    pub fn current_hole_size(&self) -> usize {
        return self.cursor as usize - self.limit as usize;
    }

    unsafe fn write<T>(&mut self, object: T, offset: usize) -> Option<*const T> {
        let p = self.block.as_ptr().add(offset) as *mut T;
        write(p, object);
        return Some(p);
    }
}

pub struct BlockList {
    /// Current block being written to
    head: Option<BumpBlock>,     

    /// Overflow so that an object that does not fit into just the head
    /// has some space
    overflow: Option<BumpBlock>,

    /// Remainder of blocks
    rest: Vec<BumpBlock>,
}

impl BlockList {
    pub fn new() -> BlockList {
        return BlockList {
            head: None,
            overflow: None,
            rest: Vec::new(),
        };
    }

    fn overflow_alloc(&mut self, alloc_size: usize) -> Result<*const u8, AllocError> {
        let space = match self.overflow {
            Some(ref mut overflow)  => {
                match overflow.inner_alloc(alloc_size) {
                    Some(space) => {
                        space
                    },
                    None => {
                        let previous = replace(overflow, BumpBlock::new()?);
                        self.rest.push(previous);

                        overflow.inner_alloc(alloc_size).expect("Unexpected allocation error!")
                    }
                }
            },
            None => {
                let mut overflow = BumpBlock::new()?;

                // object size < block size means we are not
                // able to fail this expectation
                let space = overflow
                    .inner_alloc(alloc_size)
                    .expect("We expected this block to fit!");

                self.overflow = Some(overflow);
                space
            }
        } as *const u8;

        return Ok(space);
    }
}

pub struct StickyImmixHeap<H> {
    blocks: UnsafeCell<BlockList>,
    _header_types: PhantomData<*const H>,
}

impl<H> StickyImmixHeap<H> {
    pub fn new() -> StickyImmixHeap<H> {
        return StickyImmixHeap{
            blocks: UnsafeCell::new(BlockList::new()),
            _header_types: PhantomData
        };
    }

    fn find_space(
        &self,
        alloc_size: usize,
        size_class: SizeClass
    ) -> Result<*const u8, AllocError> {
        let blocks = unsafe { &mut *self.blocks.get() };

        // TODO: handle large objects
        if size_class == SizeClass::Large {
            return Err(AllocError::BadRequest);
        }

        let space = match blocks.head {
            // We already have a block to try to use...
            Some(ref mut head) => {
                // If this is a medium object that doesn't fit in the hole, use overflow
                if size_class == SizeClass::Medium && alloc_size > head.current_hole_size() {
                    return blocks.overflow_alloc(alloc_size);
                }

                // This is a small object that might fit in the current block...
                match head.inner_alloc(alloc_size) {
                    // the block has a suitable hole
                    Some(space) => space,

                    // the block does not have a suitable hole
                    None => {
                        let previous = replace(head, BumpBlock::new()?);

                        blocks.rest.push(previous);

                        head.inner_alloc(alloc_size).expect("Unexpected error!")
                    }
                }
            }

            // We have no blocks to work with yet so make one
            None => {
                let mut head = BumpBlock::new()?;

                // earlier check for object size < block size should
                // mean we dont fail this expectation
                let space = head
                    .inner_alloc(alloc_size)
                    .expect("We expected this object to fit!");

                blocks.head = Some(head);

                space
            }
        } as *const u8;

        return Ok(space);
    }
}
