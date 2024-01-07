use futures::channel::mpsc::{unbounded, UnboundedSender};
use futures::future::Future;
use futures::stream::StreamExt;
use glib::source::Priority;
use gtk::glib;
use std::pin::Pin;

pub type FutureTask = Pin<Box<dyn Future<Output = ()> + Send>>;
pub type FutureLocalTask = Pin<Box<dyn Future<Output = ()>>>;

// The Worker (see below) is a glorified way to send an async task to the GLib(rs) future executor
pub fn spawn_task_handler(context: &glib::MainContext) -> Worker {
    let (future_local_sender, future_local_receiver) = unbounded::<FutureLocalTask>();
    context.spawn_local_with_priority(
        Priority::DEFAULT_IDLE,
        future_local_receiver.for_each(|t| t),
    );

    let (future_sender, future_receiver) = unbounded::<FutureTask>();
    context.spawn_with_priority(Priority::DEFAULT_IDLE, future_receiver.for_each(|t| t));

    Worker(future_local_sender, future_sender)
}

// Again, fancy name for an MPSC sender
// Actually two of them, in case you need to send local futures (no Send needed)
#[derive(Clone, Debug)]
pub struct Worker(
    UnboundedSender<FutureLocalTask>,
    UnboundedSender<FutureTask>,
);

impl Worker {
    pub fn send_local_task<T: Future<Output = ()> + 'static>(&self, task: T) -> Option<()> {
        self.0.unbounded_send(Box::pin(task)).ok()
    }

    pub fn send_task<T: Future<Output = ()> + Send + 'static>(&self, task: T) -> Option<()> {
        self.1.unbounded_send(Box::pin(task)).ok()
    }
}
