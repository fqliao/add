pub struct Post {
    // Why Option? can set None, let invalid old state.rs and compare conveniently
    // Why Box smart pointer? use trait object
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
        //        &self.state.rs.as_ref().unwrap().add_text(self, text);
    }
    pub fn request_review(&mut self) {
        // take() : set State to None ad-hoc, let old State invalid
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
    pub fn content(&self) -> &str {
        // due to &self, use as_ref: return Option<&Box<State>>, get a ref for State
        // unwrap: return &Box<State>
        self.state.as_ref().unwrap().content(&self)
    }
}

/// State trait
trait State {
    fn add_text(&self, post: &mut Post, text: &str) {}
    /// state.rs change fun: request_review
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    /// state.rs change fun: approve
    fn approve(self: Box<Self>) -> Box<dyn State>;
    /// State logic in content
    /// note: trait fun need self parameter
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
struct PendingReview {}
struct Published {}

/// Draft State
impl State for Draft {
    fn add_text(&self, post: &mut Post, text: &str) {
        post.content.push_str(text);
    }
    /// Draft request_review: Draft -> PendingReview
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    /// Draft approve: Draft -> Draft
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
/// PedingReview State
impl State for PendingReview {
    /// PendingReview request_review: PendingReview -> PendingReview
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    /// PendingReview approve: PendingReivew -> Published
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}
/// Published State
impl State for Published {
    /// Published reuqest_view: Published -> Published
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    /// Published approve: Published -> Published
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        // return str: post.content.as_str()
        // return &str: &post.content
        &post.content
    }
}
