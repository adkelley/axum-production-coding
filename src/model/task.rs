use crate::ctx::Ctx;
use crate::model::base;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

// region:         — Task Types

//  Sent back from API to client
#[derive(Debug, Serialize, Clone, Fields, FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

//  Sent to the model layer for creating a new task
#[derive(Fields, Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

// Sent to the model layer for updating a task
#[derive(Fields, Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}
// endregion:      — Task Types

// region:         — TaskBmc
pub struct TaskBmc;

impl base::DbBmc for TaskBmc {
    const TABLE: &'static str = "task";
}

// Functions for Business Model Component
// (NOTE) listed in  CRUD order
impl TaskBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, task_c: TaskForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, task_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
        base::list::<Self, _>(ctx, mm).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        task_u: TaskForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, task_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}
// end region:      — TaskBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
    // #![allow(unused)]
    use crate::_dev_utils;

    use super::*;
    use anyhow::Result;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";

        // -- Exec
        let task_c = TaskForCreate {
            title: fx_title.to_string(),
        };

        let id = TaskBmc::create(&ctx, &mm, task_c).await?;

        // -- Check
        let task = TaskBmc::get(&ctx, &mm, id).await?;
        // Why we have nocapture in our cargo test command
        // println!("->> {title}");
        assert_eq!(task.title, fx_title);

        // -- Cleanup
        TaskBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let id = 999;

        // -- Exec
        let res = TaskBmc::get(&ctx, &mm, id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(crate::model::Error::EntityNotFound {
                    entity: "task",
                    id: 999
                })
            ),
            "EntityNotFound not matched"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &["test_list_ok-task 01", "test_list_ok-task 02"];

        _dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

        // -- Exec
        let tasks = TaskBmc::list(&ctx, &mm).await?;

        // -- Check
        let tasks: Vec<Task> = tasks
            .into_iter()
            .filter(|t| t.title.starts_with("test_list_ok-task"))
            .collect();
        assert_eq!(tasks.len(), 2, "number of seeded tasks");

        // -- Cleanup
        for task in tasks.iter() {
            TaskBmc::delete(&ctx, &mm, task.id).await?;
        }

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        // Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_update_ok - task 01";
        let fx_title_updated = "test_update_ok - task 01 - updated";
        let fx_task = _dev_utils::seed_tasks(&ctx, &mm, &[fx_title])
            .await?
            .remove(0); // take it out

        // -- Exec
        let task_u = TaskForUpdate {
            title: Some(fx_title_updated.to_string()),
        };
        TaskBmc::update(&ctx, &mm, fx_task.id, task_u).await?;

        // -- Check
        let task = TaskBmc::get(&ctx, &mm, fx_task.id).await?;
        assert_eq!(task.title, fx_title_updated);
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let id = 999;

        // -- Exec
        let res = TaskBmc::delete(&ctx, &mm, id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(crate::model::Error::EntityNotFound {
                    entity: "task",
                    id: 999
                })
            ),
            "EntityNotFound not matched"
        );

        Ok(())
    }
}

// endregion: --- Tests
