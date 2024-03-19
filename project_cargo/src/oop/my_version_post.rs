pub enum State {
    Draft,
    Review,
    Publish,
}

pub struct NormalPost {
    state: State,
    content: String,
}

pub trait Post {
    fn add_text(&mut self, content: &str);
    fn request_review(&mut self);
    fn approve(&mut self);
}

impl Post for NormalPost {
    fn add_text(&mut self, content: &str) {
        self.content.push_str(content);
        self.state = State::Draft;
    }

    fn request_review(&mut self) {
        self.state = State::Review;
    }

    fn approve(&mut self) {
        self.state = State::Publish;
    }
}

impl NormalPost {
    pub fn new() -> NormalPost {
        NormalPost {
            state: State::Draft,
            content: String::new(),
        }
    }
}

pub fn example1() {
    println!("Hello from Post");
}
