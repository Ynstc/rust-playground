use add_one; //pay attention that library is written with underscore (but dependencies just like folder with hyphen)
use rand;

fn main() {
    let num = 10;

    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}
