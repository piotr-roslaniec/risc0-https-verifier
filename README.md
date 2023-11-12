# https-verifier

Verify SSL sessions using RISC Zero zkVM.

## Usage

```bash
./run.sh
```

## It doesn't work

You will notice that this project doesn't actually work. The `run.bash` script will fail with an error message:

```
# Abbreviated
method_name:   = note: rust-lld: error: undefined symbol: __stack_chk_guard
method_name:           >>> referenced by aes_nohw.c:918 (crypto/fipsmodule/aes/aes_nohw.c:918)
method_name:           >>>               aes_nohw.o:(GFp_aes_nohw_ctr32_encrypt_blocks) in archive /home/piotr/Documents/projects/risc0/https-verifier/target/riscv-guest/riscv32im-risc0-zkvm-elf/release/deps/libring-8429e95c128ee8f1.rlib
method_name:           >>> referenced by aes_nohw.c:918 (crypto/fipsmodule/aes/aes_nohw.c:918)
method_name:           >>>               aes_nohw.o:(GFp_aes_nohw_ctr32_encrypt_blocks) in archive /home/piotr/Documents/projects/risc0/https-verifier/target/riscv-guest/riscv32im-risc0-zkvm-elf/release/deps/libring-8429e95c128ee8f1.rlib
method_name:           >>> referenced by poly1305.c:67 (crypto/poly1305/poly1305.c:67)
method_name:           >>>               poly1305.o:(poly1305_update) in archive /home/piotr/Documents/projects/risc0/https-verifier/target/riscv-guest/riscv32im-risc0-zkvm-elf/release/deps/libring-8429e95c128ee8f1.rlib
method_name:           
method_name:           rust-lld: error: undefined symbol: __bswapsi2
method_name:           >>> referenced by aes_nohw.c:961 (crypto/fipsmodule/aes/aes_nohw.c:961)
method_name:           >>>               aes_nohw.o:(GFp_aes_nohw_ctr32_encrypt_blocks) in archive /home/piotr/Documents/projects/risc0/https-verifier/target/riscv-guest/riscv32im-risc0-zkvm-elf/release/deps/libring-8429e95c128ee8f1.rlib
method_name:           >>> referenced by aes_nohw.c:961 (crypto/fipsmodule/aes/aes_nohw.c:961)
method_name:           >>>               aes_nohw.o:(GFp_aes_nohw_ctr32_encrypt_blocks) in archive /home/piotr/Documents/projects/risc0/https-verifier/target/riscv-guest/riscv32im-risc0-zkvm-elf/release/deps/libring-8429e95c128ee8f1.rlib
method_name:           >>> referenced by aes_nohw.c:961 (crypto/fipsmodule/aes/aes_nohw.c:961)
method_name:           >>>               aes_nohw.o:(GFp_aes_nohw_ctr32_encrypt_blocks) in archive /home/piotr/Documents/projects/risc0/https-verifier/target/riscv-guest/riscv32im-risc0-zkvm-elf/release/deps/libring-8429e95c128ee8f1.rlib
method_name:           
```

## Other resources

This repo contains a lot of forks of other projects:

- https://github.com/piotr-roslaniec/clipper/tree/hackathon
- https://github.com/piotr-roslaniec/ring-xous/tree/hackathon
- https://github.com/piotr-roslaniec/sct.rs/tree/hackathon
- https://github.com/piotr-roslaniec/webpki/tree/hackathon

Some of them (`sct.rs`, and `webpki`) contain cosmetic changes that may be replaced with a proper Cargo config. 

