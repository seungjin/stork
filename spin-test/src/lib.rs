use spin_test_sdk::{
    bindings::fermyon::spin_wasi_virt::http_handler,
    bindings::{fermyon::spin_test_virt::key_value, wasi::http},
    spin_test,
};

#[spin_test]
fn foo() {
    let a = "apple".to_string();
    assert_eq!("apple".to_string(), a);
}
