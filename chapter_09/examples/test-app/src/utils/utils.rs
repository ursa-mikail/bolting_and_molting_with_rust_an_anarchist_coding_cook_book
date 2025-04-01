use rand::Rng;
use std::ptr;
//use std::time::SystemTime;

pub fn memory_readdressing() {
    let mut rng = rand::thread_rng();

    // Correct: Work with allocated memory
    let mut mem = vec![0u8; 1024]; // Allocate 1 KB of memory

    // Pick a random offset within the allocated region
    let random_offset = rng.gen_range(0..mem.len());

    // Safe access to the allocated memory
    let ptr = &mut mem[random_offset] as *mut u8;

    unsafe {
        // Write a random value to the memory at the chosen offset
        ptr::write(ptr, rng.gen_range(0..=255));
    }

    println!("Successfully wrote to memory!");
}

pub fn data_placement_with_random_offset() {
    let mut rng = rand::thread_rng();
    
    // Allocate an array of bytes (1 KB)
    let mut mem = vec![0u8; 1024];

    // Pick a random offset within the allocated region
    let random_offset = rng.gen_range(0..mem.len());

    // Place a random byte at the chosen offset (inclusive range)
    mem[random_offset] = rng.gen_range(0..=255);

    println!("Placed random byte at offset {} in allocated memory.", random_offset);
    println!("Memory content: {:?}", mem);
}

//use memmap2::{MmapMut, MmapOptions};
use memmap2::{MmapMut};
use std::fs::OpenOptions;
//use std::io::{Write, Seek, SeekFrom};
//use std::os::unix::io::AsRawFd; // Needed for file descriptor


pub fn memory_mapping_with_multiple_regions() {
    let region_count = 8;
    let page_size = 4096;
    let file_size = (region_count * page_size) as u64;

    let file_path = "mmap_test.bin";

    // Print the file path
    println!("Using file path: {}", file_path);    

    // Open the file and ensure it is large enough for mapping
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Unable to open file");

    file.set_len(file_size).expect("Unable to set file length");

    let mut mmap_regions = Vec::new();
    
    // Map memory regions
    for i in 0..region_count {
        let _offset = (i * page_size) as u64; // intentional un-use, prefix it with an underscore: `_offset`

        let mmap = unsafe {
            MmapMut::map_mut(&file)
                .expect("Memory mapping failed")
        };
        mmap_regions.push(mmap);
    }

    let mut rng = rand::thread_rng();
    
    // Modify the mapped memory with random data
    for mmap in mmap_regions.iter_mut() {
        for j in 0..page_size {
            mmap[j] = rng.gen_range(0..=255);
        }
    }

    println!("Mapped and modified memory regions successfully!");
}


pub fn random_byte_placement_in_allocated_memory() {
    let mut rng = rand::thread_rng();

    // Allocate 1 KB memory block
    let mut mem = vec![0u8; 1024];

    // Pick a random offset
    let random_offset = rng.gen_range(0..mem.len());

    // Generate a random byte
    let random_byte = rng.gen_range(0..255);

    // Place the random byte at the chosen offset
    mem[random_offset] = random_byte;

    println!("Placed random byte {:#x} at offset {} in allocated memory.", random_byte, random_offset);
    println!("Memory content: {:?}", mem);
}

