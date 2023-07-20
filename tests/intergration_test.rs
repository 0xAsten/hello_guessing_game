use hello_guessing_game;

mod common;

#[test]
fn test_add_two() {
    common::setup();
    // only can call public API
    assert_eq!(4, hello_guessing_game::add_two(2));
}
