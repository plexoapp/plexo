use super::task::Task;
use crate::backend::engine::SDKEngine;
use std::error::Error;

trait TaskWatchers {
    fn watch_created_tasks<H>(&self, handler: H) -> Result<(), Box<dyn Error>>
    where
        H: Fn(&Task) -> Result<(), Box<dyn Error>>;
}

impl TaskWatchers for SDKEngine {
    fn watch_created_tasks<H>(&self, _handler: H) -> Result<(), Box<dyn Error>>
    where
        H: Fn(&Task) -> Result<(), Box<dyn Error>>,
    {
        // let (tx, mut rx) = watch::channel(Task::default());

        // tokio::spawn(async move {
        //     loop {
        //         let task = self.task_event_recv.recv().unwrap();
        //         // handler(&task).unwrap();
        //     }
        // });

        Ok(())
    }
}
