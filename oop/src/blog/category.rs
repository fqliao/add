pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

/// 终搞提供content方法，不提供add_text方法
impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

/// 草稿不提供content方法，提供add_text方法和审阅请求方法
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_view(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

/// 审阅稿不提供content方法和add_text方法，提供同意方法
impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
