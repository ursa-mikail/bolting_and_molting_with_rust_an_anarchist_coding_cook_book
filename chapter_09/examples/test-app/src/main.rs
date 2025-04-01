mod libs;

mod utils;

fn main() {
    println!("Main function called");
    libs::lib_function();

    utils::memory_readdressing();
    utils::data_placement_with_random_offset();
    utils::memory_mapping_with_multiple_regions();
    utils::random_byte_placement_in_allocated_memory();
        
}

/*
Main function called
Library function called
Successfully wrote to memory!
Placed random byte at offset 464 in allocated memory.
Memory content: [0, 0, 0, 0, 0, 0, 0, ... 0, 66, 0, 0, ... 0, 0, 0, 0, 0, 0, 0, 0]
Using file path: mmap_test.bin
Mapped and modified memory regions successfully!
Placed random byte 0x5d at offset 216 in allocated memory.
Memory content: [0, 0, 0, 0,... 0, 93, 0, 0, ... 0, 0]
*/