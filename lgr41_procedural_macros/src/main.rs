/* 41.0.0 procedural macros */

fn main() {
    custom_derive_macro();
    println!("Hello, world!");
}

/* 41.1.0 custom-derive macros */
use lgr41_hello_macro::HelloMacro;
use lgr41_hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn custom_derive_macro() {
    Pancakes::hello_macro();
}

/* 41.2.0 attribute-like macros */

// #[route(GET, "/")]
// fn index() {
//     //...
// }

// #[proc_macro_attribute]
// fn route(
//     attr: TokenStream, //GET, "/"
//     item: TokenStream   // fn index() -> {}
// ) -> TokenStream {
//     //...
// }

/* 41.3.0 function-like macros */

// fn function_like_macro(){

//     let sql = sql!(SELECT * FROM posts WHERE id =1)

//     #[proc_macro]
//     pub fn sql(input: TokenStream) -> TokenStream {
//         //...
//     }
// }
