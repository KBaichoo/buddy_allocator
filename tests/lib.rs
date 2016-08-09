
extern crate buddy_allocator;
extern crate core;
use core::mem;
// TODO: put integration tests here (i.e. use it as a client would / will )

#[test]
fn cs107() {
    // lol
    let memory : [u8; 65536] = [0; 65536];
    //buddy_allocator::Allocator::new(&memory[0], 512000, 1024); 
    buddy_allocator::allocator::Allocator::new(unsafe { mem::transmute(&memory[0]) } , 65536, 1024); 

}
