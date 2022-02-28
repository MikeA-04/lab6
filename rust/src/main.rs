use libc::{malloc, sbrk, free, exit};
use std::{ffi::c_void, env};
use croaring_sys::{malloc_stats};

const EXP1_NUM_MALLOCS:usize = 4096;
const EXP1_MALLOC_SIZE:usize = 100;
const EXP2_NUM_MALLOCS:usize = 100;
const EXP2_MALLOC_SIZE:usize = 4096;
const EXP3_NUM_MALLOCS:usize = 100;
const EXP3_MALLOC_SIZE:usize = 4096;
const EXP5_LARGE_OBJECT_SIZE:usize = 1024;
const EXP6_MALLOC_SIZE:usize = 1000;


fn experiment6() {
    let p: *mut c_void;
    unsafe {
        p = malloc(EXP6_MALLOC_SIZE);
    }

    // Write to addresses outside of the allocated region until it segfaults
    // Can you find the offset (value of i) where it segfaults??
    // Hint: use gdb to run the program and print local variables after the segfault
    //
    // TODO 4: use gdb to find the value of 'i' when the below for loop hits a SEGFAULT
    // Hint: use 'info locals' in gdb
    for i in 0..1000000 { 
        unsafe {
            *((p as usize + EXP6_MALLOC_SIZE + i) as *mut u8) = 'a' as u8;
        }
    }
}

fn experiment5() {
    // malloc a large enough object so that its address starts with 0x7...
    // TODO 3: increase EXP5_LARGE_OBJECT_SIZE until you find a size large enough
    let p: *mut c_void;
    unsafe {
        p = malloc(EXP5_LARGE_OBJECT_SIZE);
        free(p);
    }
    println!("Large object is at 0x{:x}", p as usize);
}

fn experiment4() {
    let mut p: *mut c_void;
    let mut p2: *mut c_void;

    // Malloc two objects of the same size
    // The addresses of the objects will be the same
    unsafe {
        p = malloc(100);
        free(p);
        p2 = malloc(100);
        free(p2);
    }
    println!("Object 1 at '0x{:x}'. Object 2 is at '0x{:x}'", p as usize, p2 as usize);

    // Malloc two objects of different sizes
    // The addresses of both objects will be different
    unsafe {
        p = malloc(100);
        free(p);
        p2 = malloc(200);
        free(p2);
    }
    println!("Object 1 at '0x{:x}'. Object 2 is at '0x{:x}'", p as usize, p2 as usize);
}

fn experiment3() {
    // This is the address of the end of heap at the start of this experiment
    let mut start_heap: *mut c_void;
    let mut prev_heap: *mut c_void;
    let mut curr_heap: *mut c_void = 0 as _;

    unsafe {
        start_heap = sbrk(0);
        prev_heap = sbrk(0);
    }

    let mut pointers: [*mut c_void; EXP3_NUM_MALLOCS] = [0 as _; EXP3_NUM_MALLOCS];

    // Allocate many objects of the same size
    for i in 0..EXP3_NUM_MALLOCS { 
        unsafe {
            pointers[i] = malloc(EXP3_MALLOC_SIZE);

            curr_heap = sbrk(0);
        }
        // Check to see if the heap has changed
        if curr_heap != prev_heap {
            println!("Heap changed from 0x{:x} to 0x{:x} after {} allocations", prev_heap as usize, curr_heap as usize, i+1);
            println!("Heap size change {} bytes", curr_heap as usize - prev_heap as usize);
        }
        prev_heap = curr_heap;
    }

    println!("Total heap change after allocations: {} bytes", curr_heap as usize - start_heap as usize);

    // Deallocate all chunks in reverse order of allocation.
    //
    // Question:
    //   At which deallocation in the below loop does the heap size finally reduce?
    //   Compare this to the result of experiment2. Why do you think there is a difference?
    //   Hint: since we're deallocating in reverse order, we free objects closer to the end of the
    //   heap first.
    unsafe {
        start_heap = sbrk(0);
    }
    for i in (0..EXP3_NUM_MALLOCS).rev() { 
        unsafe {
            free(pointers[i]);

            curr_heap = sbrk(0);
        }
        // Check to see if the heap has changed
        if curr_heap != prev_heap {
            println!("Heap changed from 0x{:x} to 0x{:x} after deallocating the object at index {}", prev_heap as usize, curr_heap as usize, i);
            println!("Heap size decreased by {} bytes", prev_heap as usize - curr_heap as usize);
        }
        prev_heap = curr_heap;
    }

    unsafe {
        malloc_stats();
    }
    println!("Total heap change after deallocations: {} bytes", start_heap as usize - curr_heap as usize);
}

fn experiment2() {
    let mut start_heap: *mut c_void;
    let mut prev_heap: *mut c_void;
    let mut curr_heap: *mut c_void = 0 as _;

    unsafe {
        start_heap = sbrk(0);
        prev_heap = sbrk(0);
    }

    let mut pointers: [*mut c_void; EXP2_NUM_MALLOCS] = [0 as _; EXP2_NUM_MALLOCS];

    // Allocate many objects of the same size
    for i in 0..EXP2_NUM_MALLOCS {
        unsafe {
            pointers[i] = malloc(EXP2_MALLOC_SIZE);

            curr_heap = sbrk(0);
        }

        // Check to see if the heap has changed
        if curr_heap != prev_heap {
            println!("Heap changed from 0x{:x} to 0x{:x} after {} allocations", prev_heap as usize, curr_heap as usize, i+1);
            println!("Heap size increased by {} bytes", curr_heap as usize - prev_heap as usize);
        }
        prev_heap = curr_heap;
    }

    println!("Total heap change after allocations: {} bytes", curr_heap as usize - start_heap as usize);

    // Deallocate all chunks in order of allocation.
    //
    // Question:
    //   At which deallocation in the below loop does the heap size finally reduce?
    unsafe {
        start_heap = sbrk(0);
    }
    for i in 0..EXP2_NUM_MALLOCS { 
        unsafe {
            // free releases an a region of memory that was allocated earlier
            // Read: https://man7.org/linux/man-pages/man1/free.1.html
            // Question:
            // To free a region of memory, what argument should be passed to free?
            // TODO 2: Replace free argument 0 with the correct argument once you've figured it out.
            // Hint: we're trying to free the object pointed to by pointers[i]
            free(0 as *mut c_void);

            curr_heap = sbrk(0);
        }
        // Check to see if the heap has changed
        if curr_heap != prev_heap {
            println!("Heap changed from 0x{:x} to 0x{:x} deallocating object at index {}", prev_heap as usize, curr_heap as usize, i);
            println!("Heap size decreased by {} bytes", prev_heap as usize - curr_heap as usize);
        }
        prev_heap = curr_heap;
    }
    println!("Total heap change after all deallocations: {} bytes", start_heap as usize - curr_heap as usize);
}

fn experiment1() {
    // start_heap is the address of the end of the heap at the start of this experiment
    let start_heap;
    let mut prev_heap;
    let mut curr_heap:usize = 0;

    unsafe{
        // Read https://linux.die.net/man/2/sbrk to understand what sbrk does
        // Question:
        //   Why does passing in 0 to sbrk return the end of the heap?
        start_heap = sbrk(0) as usize;
        prev_heap = sbrk(0) as usize;
    }

    // The loop below continuously allocates memory of a fixed size
    // 
    // The local variables, prev_heap and curr_heap track the address of the end of the heap
    // through each iteration. curr_heap is used to store the end of the heap after the current allocation
    // and prev_heap stores the heap end of the last allocation. We then compare the old and new heap size
    // to figure out when the heap changes.
    //
    // Try to answer the following:
    //   At which allocation does the heap size first change?
    //   Is the change in heap size greater than or equal to the size of the allocated object?
    for i in 0..EXP1_NUM_MALLOCS {
        unsafe{
            // malloc allocates a region of memory of the provided size
            // Read: https://man7.org/linux/man-pages/man3/malloc.3.html
            //
            // Question:
            //   What does malloc return?
            malloc(EXP1_MALLOC_SIZE);
            // TODO 1: replace 0 with the correct function to get current heap end
            curr_heap = 0 as usize;
        }

        // Check to see if the heap has changed
        if curr_heap != prev_heap {
            println!("Heap changed from 0x{:x} to 0x{:x} after {} allocations", prev_heap, curr_heap, i+1);
            println!("Heap size increased by {} bytes", curr_heap - prev_heap);
            prev_heap = curr_heap;
        }
    }

    println!("Total heap change: {} bytes", curr_heap - start_heap);
}

fn print_usage() {
    println!("invalid input\n\nUsage: test_malloc <experiment number>\n\nExample: ./test_malloc 5\nThe above runs example 5.");
    unsafe { exit(0) }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
    }

    match &*args[1] {
        "1" => experiment1(),
        "2" => experiment2(),
        "3" => experiment3(),
        "4" => experiment4(),
        "5" => experiment5(),
        "6" => experiment6(),
        _ => print_usage()
    }
}
