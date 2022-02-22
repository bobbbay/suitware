// Re-export the current tokio version
pub use async_trait;
pub use tokio;

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
        #[error("no subscriptions to receive")]
        NoSubscriptions,

        #[error("could not receive from channel")]
        ChannelReceiveError(#[from] async_channel::RecvError),

        #[error("error")]
        Other(#[from] anyhow::Error),
    }

    /// A typed [`std::result::Result`] wrapper for [`enum@self::Error`].
    pub type Result<T> = std::result::Result<T, Error>;
}

pub mod context {
    //! Application contexts.

    use async_channel::{Receiver, Recv, Sender};
    use backoff::{future::retry, ExponentialBackoff};
    use futures::future::select_all;
    use halfbrown::HashMap;

    use crate::{error::Error, error::Result, message::Message, worker::Worker};

    #[derive(Clone)]
    /// An Aparan context, containing information about the worker.
    pub struct Context<'a> {
        /// Internally managed channels.
        channels: HashMap<&'a str, (Sender<Message<'a>>, Receiver<Message<'a>>)>,
        /// A list of subcribed channels in the current context.
        subscribed: Vec<&'static str>,
    }

    impl<'a> Context<'a> {
        /// Create a default instance of a context with a message type of MT.
        pub fn new() -> Self {
            Self {
                channels: HashMap::new(),
                subscribed: Vec::new(),
            }
        }

        /// Start a worker.
        pub async fn start(&self, worker: impl Worker) -> Result<()> {
            worker.start(self).await
        }

        /// Adds a new channel.
        pub async fn new_channel(&mut self, topic: &'static str) {
            let (r, s) = async_channel::unbounded();

            self.channels.insert(topic, (r, s));
        }

        pub async fn send(&mut self, topic: &str, msg: Message<'a>) -> Result<()> {
            // FIXME: Remove the unwrap. Sort it out with thiserror.
            self.channels[topic].0.send(msg).await.unwrap();

            Ok(())
        }

        #[async_recursion::async_recursion]
        pub async fn receive(&self) -> Result<Message<'a>> {
            retry(ExponentialBackoff::default(), || async {
                // Get all of the receivers we have stored out.
                let futures: Vec<Recv<_>> = self
                    .channels
                    .iter()
                    .filter(|(k, _)| self.subscribed.contains(k))
                    .map(|(_, v)| v.1.recv())
                    .collect();

                // If we have subscribed to nothing but are asking to receive,
                // try again.
                if futures.is_empty() {
                    return Err(backoff::Error::transient(Error::NoSubscriptions));
                }

                // FIXME: Parse into our own message struct
                Ok(select_all(futures)
                    .await
                    .0
                    .map_err(backoff::Error::transient)
                    .unwrap())
            })
            .await
        }

        /// Like `receive`, but receives only on one topic. Disregards subscribed topics.
        pub async fn receive_on(&self, topic: &str) -> Result<Message<'a>> {
            self.channels[topic]
                .1
                .recv()
                .await
                .map_err(|e| Error::Other(e.into()))
        }

        pub fn subscribe(&mut self, topic: &'static str) -> Result<()> {
            // FIXME
            self.subscribed.push(topic);

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

        async fn start(mut self, ctx: &Context) -> Result<()>;
    }
}

pub mod message {
    use std::fmt::Debug;

    #[derive(Debug)]
    pub struct Message<'a> {
        body: &'a dyn Sendable,
    }

    impl<'a> Message<'a> {
        pub fn new(body: &'a dyn Sendable) -> Self {
            Self { body }
        }

        pub fn get_body(&self) -> &'a dyn Sendable {
            self.body
        }
    }

    pub trait Sendable: Debug + Send + Sync {}

    impl Sendable for String {}
    impl Sendable for i32 {}
}
