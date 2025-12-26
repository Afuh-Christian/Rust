// pub trait Draw {
//     fn draw(&self);
// }


// // pub struct Screen {
// //     pub components: Vec<Box<dyn Draw>>,
// // }

// // impl Screen {
// //     pub fn run(&self) {
// //         for component in self.components.iter() {
// //             component.draw();
// //         }
// //     }
// // }


// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }


// pub struct Button {
//     pub width: u32,
//     pub height: u32,
//     pub label: String,
// }

// impl Draw for Button {
//     fn draw(&self) {
//         // code to actually draw a button
//     }
// }

// pub struct SelectBox {
//     width: u32,
//     height: u32,
//     options: Vec<String>,
// }

// impl Draw for SelectBox {
//     fn draw(&self) {
//         // code to actually draw a select box
//     }
// }


pub struct Post {
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
}




impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        ""
    }



    // --snip--
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }



    // --snip--
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}