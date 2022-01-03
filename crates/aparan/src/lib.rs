// Re-export the current tokio version
pub use tokio;
pub use async_trait;

// Top-level values we want even outside of [`prelude`].
pub use macros::{node, worker};

pub mod prelude {
    //! A re-export of all important Aparan types.

    pub use super::context::Context;
    pub use super::error::{Error, Result};
    pub use super::macros::node;
    pub use super::worker::Worker;
}

#[cfg(feature = "macros")]
pub mod macros {
    //! Aparan-macros reexports.

    pub use aparan_macros::{node, worker};
}

pub mod error {
    //! Application-specific errors.

    use thiserror::Error;

    /// An application-specific error.
    #[derive(Error, Debug)]
    pub enum Error {
        #[error("Example error")]
        ExampleError,
    }

    /// A typed [`std::result::Result`] wrapper for [`enum@self::Error`].
    pub type Result<T> = std::result::Result<T, Error>;
}

pub mod context {
    //! Application contexts.

    use crate::{
        error::Result,
    };

    /// An Aparan context, containing information about the worker.
    pub struct Context {}

    impl Context {
        /// Start a worker.
        pub async fn start<MT>(
            &self,
            worker: &mut dyn crate::worker::Worker<Message = MT>,
        ) -> Result<()> {
            worker.start(self).await
        }

        pub async fn send(&mut self, _topic: &str, _msg: &str) -> Result<()> {
            // TODO: Fill this in.

            Ok(())
        }
    }
}

pub mod worker {
    //! Workers that run on nodes.

    use crate::{context::Context, error::Result};

    /// A generic enum for workers. Can either by a sender or a receiver.
    #[async_trait::async_trait]
    pub trait Worker {
        type Message;

        async fn start(&mut self, ctx: &Context) -> Result<()>;
    }
}
