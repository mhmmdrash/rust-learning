// #[derive(Clone)]
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String
}

struct Draft {}
struct PendingReview {}
struct Published {}

struct ReviewedOnce {}

trait State {
    fn add_text(self: Box<Self>, _text: &str) -> &str {
        ""
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve_1(self: Box<Self>) -> Box<dyn State>;
    fn approve_2(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

impl State for Published {

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve_1(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve_2(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

impl State for PendingReview {


    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve_1(self: Box<Self>) -> Box<dyn State> {
        Box::new(ReviewedOnce {})
    }

    fn approve_2(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

impl State for Draft {
    fn add_text(self: Box<Self>, text: &str) -> &str{
        text
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    
    fn approve_1(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve_2(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

impl State for ReviewedOnce {


    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve_1(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve_2(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        // self.content.push_str(text);
        if let Some(p) = self.state.take() {
            self.content.push_str(p.add_text(text))
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(p) = self.state.take() {
            self.state = Some(p.request_review())
        }
    }

    pub fn approve_1(&mut self) {
        if let Some(p) = self.state.take() {
            self.state = Some(p.approve_1())
        }
    }

    pub fn approve_2(&mut self) {
        if let Some(p) = self.state.take() {
            self.state = Some(p.approve_2())
        }
    }

    pub fn reject(&mut self) {
        if let Some(p) = self.state.take() {
            self.state = Some(p.reject())
        }
    }
}