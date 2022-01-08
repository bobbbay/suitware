//! A basic node that both receives and sends messages.

use aparan::message::Message;
use aparan::prelude::*;
use aparan::tokio::time::{sleep, Duration};

#[aparan::node]
// async fn main(mut ctx: Context) -> Result<()> {
async fn main() -> Result<()> {
    let mut ctx = Context::new();

    ctx.new_channel("topic").await;

    let message = Message::new(&100_i32);
    ctx.send("topic", message).await?;

    ctx.subscribe("topic")?;

    ctx.start(MyReceiver).await?;
    ctx.start(MySender).await?;

    loop {}
}

/// A basic sender that sends a message every second.
/// Note that we can keep state inside workers.
struct MySender;

// TODO this worker actually does nothing now. This is because we cannot
// distribute a shared mutable reference to multiple workers. We need to
// use some sort of locking mechanism (maybe implicit?).

#[aparan::worker]
impl Worker for MySender {
    type Message = String;

    async fn start(mut self, _: &Context) -> Result<()> {
        let _message = Message::new(&"message".to_string());

        loop {
            // ctx.send("topic", message).await?;
            dbg!("");
            sleep(Duration::from_secs(1)).await;
        }
    }
}

/// A basic receiver that prints and responds to everything it receives.
struct MyReceiver;

#[aparan::worker]
impl Worker for MyReceiver {
    type Message = String;

    async fn start(mut self, ctx: &Context) -> Result<()> {
        // ctx.subscribe("topic")?;

        loop {
            // `ctx.receive()` will only return when it receives a message.
            let msg = ctx.receive().await?;
            dbg!(msg);

            // The type of `msg` is `Message<Self::Message>`. We can get the
            // topic with .topic(), the body with .body(), or print the whole
            // message out in Debug.
            //            println!("Address: {}, Received: {:?}", msg.topic(), msg);

            // Echo the message back.
            //           ctx.send(msg.topic(), msg.body()).await;
        }
    }
}
