// TODO: based on what we just learned about ownership, it sounds like immutable references
//   are a good fit for our accessor methods.
//   Change the existing implementation of `Ticket`'s accessor methods to take a reference
//   to `self` as an argument, rather than taking ownership of it.

pub struct Ticket {
    标题: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(标题: String, description: String, status: String) -> Ticket {
        if 标题.is_empty() {
            panic!("标题 cannot be empty");
        }
        if 标题.len() > 50 {
            panic!("标题 cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        Ticket {
            标题,
            description,
            status,
        }
    }

    pub fn 标题(&self) -> &String {
        &self.标题
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;

    #[test]
    fn works() {
        let ticket = Ticket::new("A 标题".into(), "A description".into(), "To-Do".into());
        // If you change the signatures as requested, this should compile:
        // we can call these methods one after the other because they borrow `self`
        // rather than taking ownership of it.
        assert_eq!(ticket.标题(), "A 标题");
        assert_eq!(ticket.description(), "A description");
        assert_eq!(ticket.status(), "To-Do");
    }
}
