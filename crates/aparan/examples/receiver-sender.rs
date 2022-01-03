//! A basic node that both receives and sends messages.

use aparan::prelude::*;
use aparan::tokio::time::{sleep, Duration};

#[aparan::node]
// async fn main(mut ctx: Context) -> Result<()> {
async fn main() -> Result<()> {
    let ctx = Context {};

    ctx.start(&mut MyReceiver).await?;
    ctx.start(&mut MySender).await?;

    loop {}
}

/// A basic sender that sends a message every second.
/// Note that we can keep state inside workers.
struct MySender;

#[aparan::worker]
impl Worker for MySender {
    type Message = String;

    async fn start(&mut self, ctx: &Context) -> Result<()> {
        loop {
            ctx.send("topic", "message").await?;
            sleep(Duration::from_secs(1));
        }
    }
}

/// A basic receiver that prints and responds to everything it receives.
struct MyReceiver;

#[aparan::worker]
impl Worker for MyReceiver {
    type Message = String;

    async fn start(&mut self, ctx: &Context) -> Result<()> {
        ctx.subscribe("topic").await?;

        loop {
            // `ctx.receive()` will only return when it receives a message.
            let msg = ctx.receive().await;

	    // The type of `msg` is `Message<Self::Message>`. We can get the
	    // topic with .topic(), the body with .body(), or print the whole
	    // message out in Debug.
            println!("Address: {}, Received: {:?}", msg.topic(), msg);

	    // Echo the message back.
            ctx.send(msg.topic(), msg.body()).await;
        }
    }
}
