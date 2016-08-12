#![crate_name = "buddy_allocator"]
#![crate_type = "lib"]
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod allocator;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
