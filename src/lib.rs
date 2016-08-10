#![crate_name = "buddy_allocator"]
//#![crate_type = "lib"]
// TODO: uncomment and remvoe all printlns
//#![no_std]

extern crate core;

pub mod allocator;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
