
use net_decode::test_support::{check, NYA_DSB};
use net_decode::tls::inorder_test;
pub fn read_inputs() {
    check(
        expect_test::expect_file!("../test_output/nya_dsb_inorder"),
        & inorder_test(NYA_DSB),
    );
}