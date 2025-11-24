use std::fmt;
use uuid::Uuid;

#[derive(Clone, Copy, Debug)]
pub struct RequestId(pub Uuid);

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
