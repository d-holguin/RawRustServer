use std::error::Error;
use std::fmt;

pub struct AnyErr {
    message: String,
    source: Option<Box<dyn Error>>,
}

impl std::fmt::Debug for AnyErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<std::io::Error> for AnyErr {
    fn from(error: std::io::Error) -> Self {
        AnyErr {
            message: error.to_string(),
            source: Some(Box::new(error)),
        }
    }
}

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "Error: {}", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

impl AnyErr {
    pub fn new<M: Into<String>>(message: M) -> Self {
        AnyErr {
            message: message.into(),
            source: None,
        }
    }

    pub fn wrap<E: Error + 'static>(message: String, error: E) -> Self {
        AnyErr {
            message,
            source: Some(Box::new(error)),
        }
    }
}

impl fmt::Display for AnyErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AnyErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|b| b.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anyerr_error_chain_wrapping() {
        let first_error = AnyErr::new("First Level Error");
        let second_error = AnyErr::wrap("Second Level Error".to_string(), first_error);
        let top_error = AnyErr::wrap("Top Level Error".to_string(), second_error);

        assert_eq!(format!("{}", top_error), "Top Level Error");

        assert_eq!(
            format!("{:?}", top_error).trim(),
            "Top Level Error\nCaused by:\n\tSecond Level Error\nCaused by:\n\tFirst Level Error"
        );

        println!("Display: {}", top_error);
        println!("Debug: {:?}", top_error);
    }
}
