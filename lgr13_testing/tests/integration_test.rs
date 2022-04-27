use lgr13_testing;

mod common;

#[test]
fn it_adds_two3(){
    common::setup();
    assert_eq!(4, lgr13_testing::add_two3(2));
}

// run only integration test:
// `cargo test --test integration_test`

//lib.rs -> library crate
//main.rs -> binary crate we can't directly test  binary crates with integration tests
