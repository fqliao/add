use oop::blog::category;
use oop::blog::state;

fn main() {
    //    call_state();
    call_category();
}

// good
fn call_category() {
    let mut post = category::Post::new();
    post.add_text("Hello");
    let post = post.request_view();
    let post = post.approve();
    println!("catogory: {:?}", post.content());
}

fn call_state() {
    let mut post = state::Post::new();
    let content = "I ate salad for lunch today";
    post.add_text(content);
    println!("Draft: {}", post.content());
    post.request_review();
    //    post.add_text("hello");
    println!("PendingReview: {}", post.content());
    post.approve();
    //    post.add_text("world");
    println!("Published: {}", post.content());
}
