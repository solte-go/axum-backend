use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result}; 
use sqlb::HasFields;
use sqlx::{FromRow, Row, Transaction, Postgres};
use sqlx::postgres::PgRow;

use super::projects::{Project, GetProject};

pub trait DBModelController {
    const TABLE: &'static str;
}

pub async fn create<MC, E>(_ctx: &Ctx, mm: &ModelManager, data: E) -> Result<i64>
where
    MC: DBModelController,
    E: HasFields,
{
    let db = mm.db();

    let fields = data.not_none_fields();
    let(id,) = sqlb::insert()
        .table(MC::TABLE)
        .data(fields)
        .returning(&["id"])
        .fetch_one::<_, (i64,)>(db)
        .await?;
  
    Ok(id)
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where  
    MC: DBModelController,
    E: for<'r>FromRow<'r, PgRow> + Unpin + Send,
    E: HasFields,
{
    let db = mm.db();

    // let sql = format!("SELECT * FROM {} WHERE id = $1", MC::TABLE);
    let entity: E = sqlb::select()
        .table(MC::TABLE)
        .columns(E::field_names())
        .and_where("id", "=", id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntryNotFound { entry: MC::TABLE, id })?;
    Ok(entity)
}

pub async fn get_project_by_id<MC>(_ctx: &Ctx, tx: &mut Transaction<'_, Postgres>, id: i64) -> Result<Project>
where  
    MC: DBModelController,
{
    let sql = format!("SELECT * FROM {} WHERE id = $1", MC::TABLE);
    
    let project: GetProject = sqlx::query_as(&sql)
    .bind(id)
    .fetch_one(&mut **tx)
    .await.map_err(|_| {
        Error::EntryNotFound { entry: MC::TABLE, id }
    })?;


    let select_query = sqlx::query(
        "SELECT name FROM tags WHERE project_id = $1");

        let rows = select_query.bind(project.id).fetch_all(&mut **tx).await?;
		
        let tags: Vec<String> = rows.iter().map(|r|r.get::<String, _>("name").to_string()).collect::<Vec<String>>();


        // let tags: Vec<String> = select_query.bind(project.id).map(|row| row.get::<String, _>("tag_name"))..fetch_all(&mut *tx).await?;

        //.map(|r| format!("{} - {}", r.get::<i64, _>("id"), r.get::<String, _>("name")))
		// .collect::<Vec<String>>()
        // let select_query = sqlx::query("SELECT id, name FROM ticket");
        // let tickets: Vec<Ticket> = select_query
        //     .map(|row: PgRow| Ticket {
        //         id: row.get("id"),
        //         name: row.get("name"),
        //     })
        //     .fetch_all(&pool)
        //     .await?;

    // let tags: Vec<String> = sqlx::query_as("Select tag_name FROM project_tags where project_id='$1'")
    //     .bind(project.id).fetch_all(&mut *tx)
    //     .await?;

    // if tx.commit().await.is_err() {
    //     return Err(Error::EntryNotFound { entry: MC::TABLE, id })
    // }

    let p:Project = Project { id: project.id, title: project.title, content: project.content, stack: tags };

    Ok(p)
}



pub async fn list<MC, E>(_ctx: &Ctx, mm: &ModelManager ) -> Result<Vec<E>>
where  
    MC: DBModelController,
    E: for<'r>FromRow<'r, PgRow> + Unpin + Send,
    E: HasFields,
{
    let db = mm.db();
 
    let entities: Vec<E> = sqlb::select()
        .table(MC::TABLE)
        .columns(E::field_names())
        .order_by("id")
        .fetch_all(db)
        .await?;
    Ok(entities)
}

pub async fn update<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64,  data: E) -> Result<()>
where
    MC: DBModelController,
    E: HasFields,
{
    let db = mm.db();
    let fields = data.not_none_fields();
    let count = sqlb::update()
        .table(MC::TABLE)
        .and_where("id", "=", id)
        .data(fields) 
        .exec(db)
        .await?;

    if count == 0 {
        return Err(Error::EntryNotFound { entry: MC::TABLE, id })
    }
  
    Ok(())
}
 
pub async fn delete<MC>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
where
    MC: DBModelController,
{
    let db = mm.db();
    let count = sqlb::delete()
        .table(MC::TABLE)
        .and_where("id", "=", id)
        .exec(db)
        .await?;

    if count == 0 {
        return Err(Error::EntryNotFound { entry: MC::TABLE, id })
    }
  
    Ok(())
}