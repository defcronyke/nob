use super::{NobResultError, NobResultErrorCode, NobResultSuccess};

pub trait NobApp {
    #[allow(dead_code)]
    fn main(&self) -> Result<NobResultSuccess, (NobResultError, NobResultErrorCode)> {
        Err((
            "You need to implement `NobApp::main()`, otherwise \
you will be seeing this error message right now, and nothing \
useful is going to happen."
                .to_string(),
            1, // NobResultErrorCode
        ))
    }
}
