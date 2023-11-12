#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

use net_decode::test_support::{check, NYA_DSB};
use net_decode::tls::inorder_test;
pub fn read_inputs() {
    // check(
    //     expect_test::expect_file!("../../test_output/nya_dsb_inorder"),
    //     & inorder_test(NYA_DSB),
    // );
    inorder_test(NYA_DSB);
}

pub fn main() {
    read_inputs();
}
