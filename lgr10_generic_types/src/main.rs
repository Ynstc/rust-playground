fn main() {
    // the_largest_number();
    // points_example();
    // impl_generic();
    mix_points();
}

/* 10.0 Generic types example */
fn the_largest_number() {
    let number_list = vec![24, 112, 43, 4, 95];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);


    let char_list = vec!["y", "w", "f", "g"];
    let largest = get_largest(char_list);
    println!("The largest char is {}", largest);
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

/* 10.1 Generic types in struct */
#[derive(Debug)]
struct Point{
    x: i32,
    y: i32,
}
// then
#[derive(Debug)]
struct PointA<T> {
    x: T,
    y: T,
}
// then
#[derive(Debug)]
struct PointB<T,U>{
    x: T,
    y: U
}

fn points_example() {
    let p1 = Point{x:5, y:10};
    let p2 = PointA{x:5.2, y:10.3};
    let p3 = PointB{x:5.2, y:10};

    println!("{:?},\n{:?},\n{:?}", p1, p2, p3);
}

/* 10.2 implementations example*/

struct PointG<T> {
    x:T,
    y:T,
}

impl<U> PointG<U> {
    fn x(&self) -> &U {
        &self.x
    }
}


impl PointG<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

fn impl_generic() {
    let p = PointG { x: 5, y: 10 };
    p.x(); //take a look what methods are available
    let p1 = PointG { x: 5.0, y: 10.7 };
    p1.y(); //take a look what methods are available
}

/* 10.4 generic example mixup */
struct PointK<T,U> {
    x: T,
    y: U
}

impl<T,U> PointK<T,U> {
    fn mixup<V,W>(self, other: PointK<V,W>) -> PointK<T, W> {
        PointK {
            x: self.x,
            y: other.y
        }
    }
}

fn mix_points() {
    let p1 = PointK { x: 5, y: 10.4 };
    let p2 = PointK { x: "Hello", y: "Wanderer"};

    let p3 = p1.mixup(p2);

    println!("
            p3.x = {},\n
            p3.y = {}
    ", p3.x, p3.y)
}

/* 10.5 generic enum compiled by rust */

enum Option<T> {
    Some(T),
    None
}

fn many_enums() {
    let integer = Option::Some(5);
    let float = Option::Some(5.0);
}

/**
 * does not affect performance
 * compiler will create two versions of enum
 * bellow output
 */

 enum Option_i32 {
    Some(i32),
    None
}


enum Option_f64 {
    Some(f64),
    None
}
fn many_enums_after_compile() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
