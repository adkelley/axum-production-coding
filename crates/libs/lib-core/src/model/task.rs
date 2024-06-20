use crate::ctx::Ctx;
use crate::model::base;
use crate::model::ModelManager;
use crate::model::Result;
use modql::field::Fields;
use modql::filter::FilterNodes;
use modql::filter::ListOptions;
use modql::filter::OpValsBool;
use modql::filter::OpValsInt64;
use modql::filter::OpValsString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region:         — Task Types

//  Sent back from API to client
#[derive(Debug, Serialize, Clone, Fields, FromRow)]
pub struct Task {
    pub id: i64,

    pub title: String,
    pub done: bool,
}

//  Sent to the model layer for creating a new task
#[derive(Fields, Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

// Sent to the model layer for updating a task
#[derive(Fields, Default, Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
    pub done: Option<bool>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct TaskFilter {
    id: Option<OpValsInt64>,

    title: Option<OpValsString>,
    done: Option<OpValsBool>,
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

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<TaskFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Task>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
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
    use serde_json::json;
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
    async fn test_list_all_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &["test_list_all_ok-task 01", "test_list_all_ok-task 02"];

        _dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

        // -- Exec
        let tasks = TaskBmc::list(&ctx, &mm, None, None).await?;

        // -- Check
        let tasks: Vec<Task> = tasks
            .into_iter()
            .filter(|t| t.title.starts_with("test_list_all_ok-task"))
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
    async fn test_list_by_filter_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &[
            "test_list_by_filter_ok-task 01.a",
            "test_list_by_filter_ok-task 01.b",
            "test_list_by_filter_ok-task 02.a",
            "test_list_by_filter_ok-task 02.b",
            "test_list_by_filter_ok-task 03",
        ];

        _dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

        // -- Exec
        let filters: Vec<TaskFilter> = serde_json::from_value(json!([
            {
                "title": {"$endsWith": ".a", "$containsAny": ["01", "02"]}
            },
            {
                "title": {"$contains": "03"}
            }
        ]))?;
        let list_options = serde_json::from_value(json!({ "order_bys": "!id"}))?;
        let tasks = TaskBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        // -- Check
        // println!("->> {tasks:#?}");
        assert_eq!(tasks.len(), 3, "number of seeded tasks");
        assert!(tasks[0].title.ends_with("03"));
        assert!(tasks[1].title.ends_with("02.a"));
        assert!(tasks[2].title.ends_with("01.a"));

        // -- Cleanup
        let tasks = TaskBmc::list(
            &ctx,
            &mm,
            Some(serde_json::from_value(json!([{
                "title": {"$startsWith": "test_list_by_filter_ok"}
            }]))?),
            None,
        )
        .await?;

        assert_eq!(tasks.len(), 5);
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
            ..Default::default()
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
