use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
    logger: &'a dyn Logger,
    pub max: usize,
}

impl<'a> Tracker<'a> {
    pub fn new(logger: &'a dyn Logger, max: usize) -> Self {
        Tracker { logger, max }
    }

    pub fn set_value(&self, value: &Rc<String>) {
        let count = Rc::strong_count(value);
        let percentage = (count * 100) / self.max;

        if count >= self.max {
            self.logger.error("Error: you are over your quota!");
        } else if percentage >= 70 {
            self.logger.warning(&format!(
                "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                percentage
            ));
        }
    }

    pub fn peek(&self, value: &Rc<String>) {
        let count = Rc::strong_count(value);
        let percentage = (count * 100) / self.max;

        self.logger.info(&format!(
            "Info: you are using up to {}% of your quota",
            percentage
        ));
    }
}
