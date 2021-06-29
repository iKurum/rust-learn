use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today.");
    println!("post add_text: {:?}", post.state());

    post.request_review();
    println!("post request_review: {:?}", post.state());
    assert_eq!("", post.content());

    post.reject();
    post.add_text("Hi.");
    println!("post reject: {:?}", post.state());

    post.request_review();
    post.approve();
    println!("post approve: {:?}", post.state());
    assert_eq!("I ate a salad for lunch today.Hi.", post.content());
}
