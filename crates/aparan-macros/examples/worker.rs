//! Uses the #[worker] attribute to create a worker.

use aparan::prelude::{Context, Worker, Result};

struct MyMiscWorker;

#[aparan::worker]
impl Worker for MyMiscWorker {
    type Message = ();

    async fn start(&mut self, _: &Context) -> Result<()> {
	Ok(())
    }
}

fn main() {}
