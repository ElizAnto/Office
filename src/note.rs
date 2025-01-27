// src/note.rs
#[derive(Debug)]
pub struct Note {
    pub title: String,
    pub content: String,
}

impl Note {
    pub fn new(title: String, content: String) -> Self {
        Note { title, content }
    }

    pub fn display(&self) {
        println!("Title: {}\nContent: {}\n", self.title, self.content);
    }
}
