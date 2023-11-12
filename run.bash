#!/bin/bash

export LIBRARY_PATH="$(pwd)/frida-gum"

# https://github.com/risc0/risc0/issues/443
#CC=clang CFLAGS_riscv32im_risc0_zkvm_elf="-target riscv32-unknown-elf" cargo build
CC=gcc CC_riscv32im_risc0_zkvm_elf=/opt/riscv/bin/riscv32-unknown-elf-gcc cargo build
