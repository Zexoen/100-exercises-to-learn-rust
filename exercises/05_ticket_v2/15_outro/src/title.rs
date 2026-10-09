// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 bytes.
//   Implement the traits required to make the tests pass too.
#[derive(thiserror::Error, Debug)]
pub enum TitleError {
    #[error("The title cannot be empty")]
    Empty,
    #[error("The title cannot be longer than 50 bytes")]
    ToLong,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TicketTitle(String);
impl std::convert::TryFrom<String> for TicketTitle {
    type Error = TitleError;
    fn try_from(s: String) -> Result<Self,Self::Error> {
        if s.is_empty() {
            Err(Self::Error::Empty)
        } else if s.len() > 50 {
            Err(Self::Error::ToLong)
        } else {
            Ok(Self(s))
        }
    }
}
impl std::convert::TryFrom<&str> for TicketTitle {
    type Error = TitleError;
    fn try_from(s: &str) -> Result<Self,Self::Error> {
        if s.is_empty() {
            Err(Self::Error::Empty)
        } else if s.len() > 500 {
            Err(Self::Error::ToLong)
        } else {
            Ok(Self(s.into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
