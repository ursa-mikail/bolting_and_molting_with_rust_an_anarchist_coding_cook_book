# Root Makefile for building the entire project

CC = gcc
CFLAGS = -Wall -Wextra -g

# Directories
RUST_DIR = rust
C_DIR = c
TARGET_DIR = $(RUST_DIR)/target/release

# Output binary
OUTPUT = rust_c_example

.PHONY: all clean rust c

all: rust c

rust:
	cd $(RUST_DIR) && cargo build --release
	mkdir -p $(RUST_DIR)/include

c: rust
	$(CC) $(CFLAGS) -o $(OUTPUT) $(C_DIR)/main.c -L$(TARGET_DIR) -lrust_with_c -Wl,-rpath,$(TARGET_DIR)

clean:
	rm -f $(OUTPUT)
	cd $(RUST_DIR) && cargo clean
