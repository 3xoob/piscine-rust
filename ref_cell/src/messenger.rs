use std::cell::RefCell;
use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

#[derive(Debug)]
pub struct Tracker<L> {
    logger: L,
    value: Rc<RefCell<u32>>,
    max: u32,
}

impl<L> Tracker<L>
where
    L: Logger,
{
    pub fn new(logger: L, max: u32) -> Self {
        Tracker {
            logger,
            value: Rc::new(RefCell::new(0)),
            max,
        }
    }

    pub fn set_value(&self, value: &Rc<RefCell<u32>>) {
        let current_value = Rc::strong_count(value) as u32;
        let percentage = (current_value as f32 / self.max as f32) * 100.0;

        if percentage >= 100.0 {
            self.logger.error("you are over your quota!");
        } else if percentage >= 70.0 {
            self.logger.warning(&format!(
                "you have used up over {:.0}% of your quota! Proceeds with precaution",
                percentage
            ));
        }
    }

    pub fn peek(&self, value: &Rc<RefCell<u32>>) {
        let current_value = Rc::strong_count(value) as u32;
        let percentage = (current_value as f32 / self.max as f32) * 100.0;
        self.logger.info(&format!(
            "you are using up to {:.0}% of your quota",
            percentage
        ));
    }
}
