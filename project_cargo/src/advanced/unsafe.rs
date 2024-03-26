use std::slice;

pub fn example1() {
    // a address in memory
    let address = 0x012345usize;
    // get a pointer to that address
    let r = address as *const i32;

    let mut num = 5;

    // more than one pointer mutable to the same memory
    let r1 = &mut num as *mut i32;
    let r2 = &mut num as *mut i32;
    // allow change a const pointer
    let r3 = &num as *const i32;

    unsafe {
        *r2 = 6;
        *r1 = 8;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        println!("r3 is: {}", *r2);
        println!("r is: {}", *r);
    }
}

pub fn example2() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

pub fn example3() {
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
}

pub fn example4() {
    // using C in Rust
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

pub fn example5() {
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    unsafe {
        COUNTER = COUNTER + 7;
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
