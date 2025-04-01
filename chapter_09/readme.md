cd examples
cargo new test-app

cd src
cargo clean
cargo update
cargo build

cargo run

/*
Main function called
Library function called
Utility function called
*/


Key Areas to Check
File Size and Permissions: Ensure that the file (mmap_test.bin) is large enough for the memory mapping. You’ve already set the file length with set_len, but make sure you also have appropriate permissions for the file.

Memory Mapping: When you are mapping the file using MmapOptions, make sure that the offset and length are correct for each region. In this case, you are correctly setting offset and len, but check whether the offsets and lengths are properly aligned to the system's page size.

Ensure Safe Access: Instead of using unsafe code, I recommend using a safer approach if possible. However, if you must use unsafe, ensure that you correctly handle memory bounds.


Check Permissions for File: Ensure that mmap_test.bin has proper read/write permissions:

`main.rs` is located in the `src` directory, and mmap_test.bin will be created in the root of the project, which is one level above the src directory.

To ensure that your program creates mmap_test.bin in the right location, make sure your path to the file in your code is just "mmap_test.bin", which will resolve to the root directory.

```

touch mmap_test.bin
chmod 666 mmap_test.bin

```

Aligning Memory Regions with System Page Size: It’s essential to ensure the memory regions are aligned to the page size. Many systems require memory regions to be aligned on page boundaries, which might be causing your segmentation fault. Consider checking the alignment and ensuring it's correct.
