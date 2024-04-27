use crate::constants::{self, *};

pub struct BlockMeta {
    lines: *mut u8,
}

impl BlockMeta {
    pub fn new(block_ptr: *const u8) -> BlockMeta {
        let meta = BlockMeta {
            lines: unsafe { block_ptr.add(constants::LINE_MARK_START) as *mut u8 },
        };

        meta.reset();

        return meta;
    }

    pub fn reset(&self) {
        unsafe {
            for i in 0..constants::LINE_COUNT {
                *self.lines.add(i) = 0;
            }
        }
    }
    
    pub fn find_next_available_hole(
        &self,
        starting_at: usize,
        alloc_size: usize
    ) -> Option<(usize, usize)> {
        // The count of consecutive holes
        // Must take into account a conservativly 
        // maked hole at the beginning of a sequence
        let mut count = 0;
        let starting_line = starting_at / LINE_SIZE;
        let lines_required = (alloc_size + LINE_SIZE -1) / LINE_SIZE;

        let mut end = starting_line;

        for index in (0..starting_line).rev() {
            let marked = unsafe { *self.lines.add(index) };

            if marked == 0 {
                count += 1;

                if index == 0 && count >= lines_required {
                    let limit = index * LINE_SIZE;
                    let cursor = end * LINE_SIZE;
                    return Some((cursor, limit));
                }
            } else {
                // Thist block is marked
                if count > lines_required {
                    // At least 2 previous blocks were unmarked.
                    // Return the hole, considering the immediately
                    // preceeding block as conservatively marked
                    let limit = (index + 2) * LINE_SIZE;
                    let cursor = end * LINE_SIZE;
                    return Some((cursor, limit));
                }
            }

            // If this line is marked and we did not return a new cursor,limit pair
            // by this point, reset the hole search state
            count = 0;
            end = index;
        }

        return None;
    }

}
