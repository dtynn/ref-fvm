use fvm_shared::error::ExitCode;

use crate::{ActorDowncast, ActorError};

pub trait Abortable<T> {
    fn or_abort(self, code: ExitCode, msg: impl AsRef<str>) -> Result<T, ActorError>;
}

impl<T, E> Abortable<T> for Result<T, E>
where
    E: ActorDowncast,
{
    fn or_abort(self, code: ExitCode, msg: impl AsRef<str>) -> Result<T, ActorError> {
        self.map_err(|e| e.downcast_default(code, msg))
    }
}

impl<T> Abortable<T> for Result<T, ExitCode> {
    fn or_abort(self, code: ExitCode, msg: impl AsRef<str>) -> Result<T, ActorError> {
        self.map_err(|c| ActorError::new(code, format!("{} with {:?}", msg.as_ref(), c)))
    }
}

pub trait IntoAnyhow<T> {
    fn anyhow(self) -> anyhow::Result<T>;
}

impl<T, E> IntoAnyhow<T> for Result<T, E>
where
    E: std::convert::Into<anyhow::Error>,
{
    fn anyhow(self) -> anyhow::Result<T> {
        self.map_err(|e| e.into())
    }
}
