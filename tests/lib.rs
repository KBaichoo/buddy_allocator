
extern crate buddy_allocator;
extern crate core;
extern crate rand;
use core::mem;
use buddy_allocator::allocator;
use std::vec;
// TODO: put integration tests here (i.e. use it as a client would / will )

// An ode to CS107's Heap Allocator.
#[test]
fn cs107() {
    let memory : [u8; 65536] = [0; 65536];
    let mut myAllocator = allocator::Allocator::new(
        unsafe { mem::transmute(&memory[0]) }, 4096, 1024);
    
    alloc_and_test(&mut myAllocator, 420, true); // should fail ( block size too small)
    alloc_and_test(&mut myAllocator, 4200, true); // should fail ( block size too large)
    
    alloc_and_test(&mut myAllocator, 2036, false);
    alloc_and_test(&mut myAllocator, 2036, false);
    alloc_and_test(&mut myAllocator, 2036, true); // should fail now (no space left)


}


// uses the entire memory location
#[test]
fn full_load() {
    let memory : [u8; 65536] = [0; 65536];
    let mut myAllocator = allocator::Allocator::new(
        unsafe { mem::transmute(&memory[0]) }, 16384, 1024);
    let mut address_vec : Vec<usize> = Vec::new();
    let mut curr_req_size = 1024;
    
    println!("Allocating memory...");
    while curr_req_size < 16384 {
        println!("Allocating chunk of size {}", curr_req_size);
        address_vec.push(alloc_and_test(&mut myAllocator, curr_req_size - 4, false).unwrap());
        curr_req_size = curr_req_size << 1;
    }

    // make an extra 1024 request
    address_vec.push(alloc_and_test(&mut myAllocator, 1020, false).unwrap());
    
    // this should fail as all the memroy should be taken...
    alloc_and_test(&mut myAllocator, 1024, true);
    address_vec.reverse();

    println!("Freeing memory");

    // free all starting from the back.
    for addr in address_vec {
        free_and_test(&mut myAllocator, addr);
    }
}

#[test]
fn small_blocks() {
    // allocs the whole memory in small blocks...
    let memory : [u8; 65536] = [0; 65536];
    let mut myAllocator = allocator::Allocator::new(
        unsafe { mem::transmute(&memory[0]) }, 16384, 512);
    let mut address_vec : Vec<usize> = Vec::new();
    let mut size_left = 16384;
    
    let start_address : usize = unsafe { mem::transmute(&memory[0])};

    println!("Allocating Blocks.... starting at address {}",  start_address);

    while size_left > 0 {
        address_vec.push(alloc_and_test(&mut myAllocator, 508, false).unwrap());
        size_left -= 512;
    }
    
    println!("Freeing blocks..."); 

    while !address_vec.is_empty() {
        let index = (address_vec.len() as f64 * rand::random::<f64>()) as usize;
        let address = address_vec.remove(index);
        println!("Removing block at address {}", address);
        free_and_test(&mut myAllocator, address);
    }

    alloc_and_test(&mut myAllocator, 14201, false); // the entire segement should be
                                                    // consolidated now, so this should
                                                    // work!

}



fn free_and_test(allocator: &mut allocator::Allocator, address: usize) {
    allocator.free(address);
    allocator.verify_lists();
    let block : &mut allocator::BlockHeader = unsafe { 
            mem::transmute(address - 4) 
    };
    assert!(block.is_free());
}

fn alloc_and_test(allocator: &mut buddy_allocator::allocator::Allocator, size : usize, 
    expected_fail : bool) -> Option<usize> {
    let results = allocator.alloc(size);
    allocator.verify_lists();
    if(expected_fail) {
        assert!(results.is_none());
    } else {
        // verify size and that it's not marked as free.
        let block : &mut allocator::BlockHeader = unsafe { 
                mem::transmute(results.unwrap() - 4) 
        };
        assert_eq!(block.get_size(), buddy_allocator::allocator::next_power_of_two((size + 4) as u32));

        assert!(!block.is_free());
    }
    results
}
