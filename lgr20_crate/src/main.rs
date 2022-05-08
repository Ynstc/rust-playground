/* 20.0 realeasing own crate */
/*
    # Two building options
    1. `cargo build` //dev
    2. 'cargo build --release` //release

    both can be customized in Cargo.toml file
*/

/* 20.1 example2 */
// use lgr20_crate::kinds::PrimaryColor;
// use lgr20_crate::utils::mix;
// because are reexported - look lib.rs
use lgr20_crate::PrimaryColor;
use lgr20_crate::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
