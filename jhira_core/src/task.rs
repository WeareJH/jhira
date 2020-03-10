use crate::async_task::{AsyncTask, TaskOutput};
use async_trait::async_trait;

pub type TaskSequence = Result<Vec<Box<dyn AsyncTask>>, failure::Error>;

pub enum Task {
    Once(Box<dyn AsyncTask>),
    Chain(Vec<Task>),
}

#[async_trait(?Send)]
impl AsyncTask for Task {
    async fn exec(&self) -> Result<TaskOutput, failure::Error> {
        exec_er(&self).await
    }
    async fn dry_run(&self) -> Result<TaskOutput, failure::Error> {
        dry_run(&self).await
    }
}

async fn exec_er(task: &Task) -> Result<TaskOutput, failure::Error> {
    match task {
        Task::Once(async_task) => async_task.exec().await,
        Task::Chain(async_tasks) => {
            let mut outputs: Vec<TaskOutput> = vec![];
            for i in async_tasks {
                let output = i.exec().await?;
                outputs.push(output);
            }
            Ok(TaskOutput::Chain(outputs))
        }
    }
}

async fn dry_run(task: &Task) -> Result<TaskOutput, failure::Error> {
    match task {
        Task::Once(async_task) => async_task.dry_run().await,
        Task::Chain(async_tasks) => {
            let mut outputs: Vec<TaskOutput> = vec![];
            for i in async_tasks {
                let output = i.dry_run().await?;
                outputs.push(output);
            }
            Ok(TaskOutput::Chain(outputs))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::async_task::AsyncTask;
    use crate::context::Context;
    use crate::issues::issues_fetch::IssuesFetch;
    use crate::task::Task;
    use std::sync::Arc;

    #[tokio::main]
    #[test]
    async fn test_task() -> Result<(), failure::Error> {
        let c = Arc::new(Context::default());
        let t1 = Task::Once(Box::new(IssuesFetch::new(c.clone())));
        let t2 = Task::Once(Box::new(IssuesFetch::new(c.clone())));

        let all_tasks = Task::Chain(vec![t1, Task::Chain(vec![t2])]);

        let _output = all_tasks.dry_run().await;

        Ok(())
    }
}
