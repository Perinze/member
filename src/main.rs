use sqlx::mysql::{MySqlPoolOptions};
use sqlx::{FromRow, Database, Pool, MySql, Error};

enum Department {
    Product,
    Design,
    Operation,
    Development,
    Magazine,
    NewMedia,
    HumanResources,
}

enum Gender {
    None,
    Male,
    Female,
    Other,
}

#[derive(FromRow, Debug)]
struct Individual {
    id: i32,
    name: String,
    department: i32,
    student_id: String,
    gender: i32,
}

struct Conn<DB: Database> {
    pool: Pool<DB>,
}

impl<DB> Conn<DB>
where
    DB: Database {

    fn new(pool: Pool<DB>) -> Self {
        Self { pool }
    }
}

impl Conn<MySql> {
    async fn individual_by_id(&self, id: i32) -> Result<Individual, Error> {
        let individual: Individual = sqlx::query_as("SELECT * FROM test where id = ?")
            .bind(id)
            .fetch_one(&self.pool).await?;
        Ok(individual)
    }
}

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:member@172.18.0.2/test").await?;

    let conn = Conn::new(pool);

    let individual = conn.individual_by_id(1).await?;

    println!("{:?}", individual);

    Ok(())
}