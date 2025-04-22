use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
mod messenger;
pub use messenger::*;

#[derive(Debug)]
pub struct Worker {
    track_value: Rc<RefCell<u32>>,
    mapped_messages: RefCell<HashMap<&'static str, String>>,
    all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(_max: u32) -> Self {
        Worker {
            track_value: Rc::new(RefCell::new(0)),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Warning", msg.to_string());
        self.all_messages
            .borrow_mut()
            .push(format!("Warning: {}", msg));
    }

    fn info(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Info", msg.to_string());
        self.all_messages
            .borrow_mut()
            .push(format!("Info: {}", msg));
    }

    fn error(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Error", msg.to_string());
        self.all_messages
            .borrow_mut()
            .push(format!("Error: {}", msg));
    }
}
