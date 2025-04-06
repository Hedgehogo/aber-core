#[expect(clippy::module_inception)]
pub mod next_stage;
pub mod custom;
pub mod or;
pub mod error;

pub use next_stage::{NextStage, SResult};
pub use custom::custom;
pub use error::Error;