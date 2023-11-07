use std::{time::Duration, fs, path::PathBuf};

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tracing::info;

use crate::{model::{ModelManager, user::{User, UserMC}}, ctx::Ctx};

type DB = Pool<Postgres>;

const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:postgres@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost/app_db"; 


// sql files

const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

//TODO Read to figure out "BOX" and "dyn"
pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - init_dev_db", "DEV-OPERATION");
    
    // will drop the scope variubles after execution
    {
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
        pexec(&root_db, SQL_RECREATE_DB).await?;
    }

    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .filter_map(|entry| entry.ok().map(|d| d.path()))
        .collect();

    paths.sort();

    let app_db = new_db_pool(PG_DEV_APP_URL).await?;
    
    for path in paths{
        if let Some(path) = path.to_str(){
            let path = path.replace('\\', "/"); // for windows compatibility

            if path.ends_with(".sql") && path != SQL_RECREATE_DB {
                pexec(&app_db, &path).await?
            }
        }
     
    }

    let mm = ModelManager::new().await?;
    let ctx = Ctx::root_ctx();

    let dev_user: User = UserMC::first_by_username(&ctx, &mm, "Pupu-The-Tester")
        .await?.unwrap();

    UserMC::update_pwd(&ctx, &mm, dev_user.id, "welcome").await?;
    info!("{:<12} - init_dev_db - set dev user pwd", "DEV-OPERATION");

    Ok(())
}


async fn new_db_pool(db_con_url: &str) -> Result<DB, sqlx::Error>{
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_con_url)
        .await
}

async fn pexec(db: &DB, file: &str) -> Result<(), sqlx::Error> {
    info!("{:<12} - postgres_exec: {file}", "DEV-OPERATION");

    let content = fs::read_to_string(file)?; 

    let sqls: Vec<&str> = content.split(';').collect();

    for sql in sqls {
         sqlx::query(sql).execute(db).await?;
    }

    Ok(())
}