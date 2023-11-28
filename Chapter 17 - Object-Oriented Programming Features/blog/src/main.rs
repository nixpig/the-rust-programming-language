use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate sausage rolls for lunch today.");

    let post = post.request_review();

    let post = post.reject();

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate sausage rolls for lunch today.", post.content());
}
