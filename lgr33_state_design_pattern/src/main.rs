/* 33.0.0 State design pattern */
use lgr33_state_design_pattern::{Post, PostA};

fn main() {
    object_oriented_state_approach();
    types_state_approach();
}

fn object_oriented_state_approach() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    println!("---\n");
}

fn types_state_approach() {
    let mut post = PostA::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    println!("---\n");
}
