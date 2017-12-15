use std::cell::RefCell;
use std::fmt::Display;

#[derive(Default)]
pub struct Errors {
    errors: RefCell<Option<Vec<String>>>,
}

impl Errors {
    pub fn new() -> Self {
        Errors {
            errors: RefCell::new(Some(Vec::new())),
        }
    }

    pub fn error<T: Display>(&self, msg: T) {
        self.errors
            .borrow_mut()
            .as_mut()
            .unwrap()
            .push(msg.to_string());
    }

    pub fn check(self) -> Result<(), String> {
        let mut errors = self.errors.borrow_mut().take().unwrap();
        match errors.len() {
            0 => Ok(()),
            1 => Err(errors.pop().unwrap()),
            n => {
                let mut msg = format!("{} errors:", n);
                for err in errors {
                    msg.push_str("\n\t# ");
                    msg.push_str(&err);
                }
                Err(msg)
            }
        }
    }
}

impl Drop for Errors {
    fn drop(&mut self) {
        if self.errors.borrow().is_some() {
            panic!("forgot to check for errors");
        }
    }
}
