use std::{fs, path::PathBuf, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

use crate::{
    ctx::Ctx,
    model::{
        user::{User, UserBmc},
        ModelManager,
    },
};

// Personal preference: alias the Pool type to Db.
type Db = Pool<Postgres>;

// NOTE: Hardcode to prevent deployment to production.
const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost/app_db";

// sql files
const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

const DEMO_PWD: &str = "welcome";

// Preference to use anyhow only on unit tests, enforcing strictness
pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - init_dev_db", "FOR-DEV-ONLY");

    // This will drop the root db at the end of the code block
    {
        // -- Create the db pool
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
        // -- Create the app root_db
        pexec(&root_db, SQL_RECREATE_DB).await?;
    }

    // -- Get the sql files
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    paths.sort();

    // -- SQL Execute each file
    let app_db = new_db_pool(PG_DEV_APP_URL).await?;
    for path in paths {
        if let Some(path) = path.to_str() {
            let path = path.replace('\\', "/"); // Windows fix

            // Only take the .sql and skip the SQL_RECREATE_DB
            if path.ends_with(".sql") && path != SQL_RECREATE_DB {
                // -- Execute the sql
                pexec(&app_db, &path).await?;
            }
        }
    }

    // -- Init model layer.
    let mm = ModelManager::new().await?;
    let ctx = Ctx::root_ctx();

    // -- Set demo1 pwd
    let demo1_user: User = UserBmc::first_by_username(&ctx, &mm, "demo1")
        .await?
        .unwrap();
    UserBmc::update_pwd(&ctx, &mm, demo1_user.id, DEMO_PWD).await?;
    info!("{:<12} - init_dev_db - set demo 1 password", "FOR-DEV-ONLY");

    Ok(())
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    info!("{:<12} - pexec: {file}", "FOR-DEV-ONLY");

    // -- Read the file
    let content = fs::read_to_string(file)?;

    // FIXME: Make the split more sql proof.
    let sqls: Vec<&str> = content.split(";").collect();

    for sql in sqls {
        // -- Execute the sql
        sqlx::query(sql).execute(db).await?;
    }

    Ok(())
}

async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_con_url)
        .await
}
