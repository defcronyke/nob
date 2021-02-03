mod app;
mod id;
mod kind;
mod obj;
mod result;
mod root;

use app::NobApp;
pub use result::{NobResultError, NobResultErrorCode, NobResultSuccess};
pub use root::NobRoot;

const NOB_NAME_DEFAULT: &str = "Unnamed";
pub const NOB_NAME: &str = "Nob";
pub const NOB_VERSION: &str = "v0.1.x";

pub fn main(app: &dyn NobApp) -> Result<NobResultSuccess, (NobResultError, NobResultErrorCode)> {
    app.main()
}
