use super::source::Source;
use crate::error::{OmgError, Position};
use tokio::{fs, prelude::Future};

pub fn loader(path: String) -> impl Future<Item = Source, Error = OmgError> {
    fs::read(path.clone()).then(|result| match result {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(source) => Ok(Source { source, path }),
            Err(err) => Err(OmgError::new(
                format!("Can't open {}, file is not valid utf8. {}", path, err),
                Position::from(path),
            )),
        },
        Err(error) => Err(OmgError::new(
            format!("Can't open {}, io error: {}", path, error),
            Position::from(path),
        )),
    })
}
