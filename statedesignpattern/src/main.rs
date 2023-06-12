mod blog;
use blog::Post;

fn main() {

    let mut post = Post::new();

    post.add_text("Hello Boss");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("Hello Boss", post.content());

}