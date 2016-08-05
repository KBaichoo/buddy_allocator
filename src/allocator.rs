/// TODO: this will be a generic allocator using a binary buddy system. 
/// Really just acting as the storage layer for now, but will fix later - Kevin.
/// NOTE: assumes that we don't start at 0x0

use core::option;
use core::mem;

const smallest_block : usize = 1024; // 1kb is the smallest size

pub struct Allocator {
    start_address : usize,
    // note: as this is a binary buddy allocator, size initalized must be a power of 2!
    size: usize, // in bytes 
    free_list: [FreeHeader; 30],
    //TODO: remove ( when I figure out how to dynamically determine FL size)
    free_list_size: u32,
    smallest_block_size: usize
}

#[derive(Copy, Clone)]
struct FreeHeader {
    header: u32,
    addr:   Option<usize>,
    next:   Option<usize>
    // a 0 for next means NULL essentially (since the lists are kept in order in
                   // ascending order.
}


// expects num to be a power of 2. Tells which power of two it is.
fn power_of_two( num : usize) -> u32 {
    let max_pos = (mem::size_of::<usize>() * 8) - 1;
    for i in max_pos..0 {
        if (1 << i) & num != 0 {
            return i as u32
        }
    }
    assert!(false); // should never reach here
    0
}
    
// Gets the next power of two (for 32 bit)
fn next_power_of_two(mut size : u32) -> u32 {
    size = size - 1;
    size |= size >> 1;
    size |= size >> 2;
    size |= size >> 4;
    size |= size >> 8;
    size |= size >> 16;
    size = size + 1;
    size
}


impl Allocator {
    
    /// constructs a new buddy allocator
    pub fn new( start_addr : usize, sz : usize, smallest_block_size : usize) -> Allocator {
        assert_eq!(sz & (sz - 1), 0); // assert size is a power of two
        let mut num_freelists = power_of_two(sz);
        //check that the number of freelist isn't too much (we're handling at most 2^30 bytes)
        assert!( num_freelists < 31);
        // include the smallest block
        num_freelists = num_freelists - power_of_two(smallest_block_size) + 1; 
        
        let mut alloc = Allocator {
            start_address: start_addr,
            size: sz,
            free_list: [FreeHeader { header : 0, addr: None, next: None  }; 30],
            free_list_size: num_freelists,
            smallest_block_size: smallest_block_size
        };
        
        // add the size of the allocator into the freelist
        alloc.free_list[num_freelists as usize - 1] = FreeHeader {
            header: (1 << 31) | (sz as u32),
            addr: Some(start_addr),
            next: None
        };

        alloc
    }
    
    // returns None on failure, Address on success
    pub fn alloc(&self, mut size: usize) -> Option<usize> {
        // return on useless request
        if size == 0 {
            return None
        }
        
        // pad sizing...
        size += mem::size_of::<u32>(); // add header size
        let padded_size = next_power_of_two(size as u32);

        // get the index to begin the search
        /*
        let mut index = self.get_freelist_index(padded_size as usize);
        let block : Option<usize> = None;

        while( block.is_none() && index < self.free_list_size) {
            // TODO: if free_list[index] is null, increment index
            if(self.free_list[index]) {
                index = index + 1;
            } else {
                // break the block originally if necessary and update the list
                //take the block, break it, split it.
            }
        }
      */
        // find an appropriate sized block
        None

    }
    

    // takes in some size, usize and returns where which index that is in the lists...
#[inline(always)]
    fn get_freelist_index(&self, size: usize) -> usize {
        (power_of_two(size) - power_of_two(self.smallest_block_size)) as usize
    }


}
