use std::fmt;

pub struct InDate {
    at: String,
}

impl fmt::Display for InDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "in data: {}", self.at,)
    }
}

pub struct ErrorData {
    in_date: InDate,
    code: String,
    message: String,
}

impl ErrorData {
    pub fn build(code: String, message: String, date: String) -> ErrorData {
        return ErrorData {
            in_date: InDate { at: date },
            code,
            message,
        };
    }
}

impl fmt::Display for ErrorData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "code: {} - msg: {} - {}",
            self.code, self.message, self.in_date,
        )
    }
}

#[allow(dead_code)]
pub enum Status {
    Draft,
    Ready(InDate),
    WorkingInProgress { step: u8, max_steps: u8 },
    Completed(InDate),
    Canceled(InDate),
    Error(ErrorData),
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Draft => write!(f, "status is Draft"),
            Status::Ready(val) => write!(f, "status is Ready at {}", val.at),
            Status::Completed(val) => write!(f, "status is Done {}", val.at),
            Status::WorkingInProgress { step, max_steps } => {
                write!(f, "status is Work in progress {}/{}", step, max_steps)
            }
            Status::Error(val) => write!(f, "status is Error {}", val),
            Status::Canceled(val) => write!(f, "status is Canceled at {}", val.at),
        }
    }
}
