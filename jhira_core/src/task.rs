use crate::async_task::{AsyncTask, TaskOutput};
use async_trait::async_trait;

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
    use crate::issues::Issues;
    use crate::task::Task;

    #[tokio::main]
    #[test]
    async fn test_task() -> Result<(), failure::Error> {
        let t1 = Task::Once(Box::new(Issues {
            url: String::from("yoyo 1"),
        }));
        let t2 = Task::Once(Box::new(Issues {
            url: String::from("yoyo 2"),
        }));
        let t3 = Task::Once(Box::new(Issues {
            url: String::from("yoyo 3"),
        }));

        let t4 = Task::Once(Box::new(Issues {
            url: String::from("yoyo 4"),
        }));
        let t5 = Task::Once(Box::new(Issues {
            url: String::from("yoyo 5"),
        }));
        let t6 = Task::Once(Box::new(Issues {
            url: String::from("yoyo 6"),
        }));
        let t7 = Task::Once(Box::new(Issues {
            url: String::from("yoyo 7"),
        }));

        let all_tasks = Task::Chain(vec![t1, t2, t3, Task::Chain(vec![t4, t5, t6, t7])]);

        let _output = all_tasks.dry_run().await;

        Ok(())
    }
}
