use sqlx::mysql::{MySqlPoolOptions};
use sqlx::Row;

enum Department {
    None,
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

#[derive(sqlx::FromRow, Debug)]
struct Individual {
    id: i32,
    name: String,
    department: i32,
    student_id: String,
    gender: i32,
}

struct Conn<DB: sqlx::Database> {
    pool: sqlx::Pool<DB>,
}

impl<DB> Conn<DB> 
where 
    DB: sqlx::Database {

    fn new(pool: sqlx::Pool<DB>) -> Self {
        Conn { pool }
    }
}

impl Conn<sqlx::MySql> {
    async fn individual_by_id(&self, id: i32) -> Result<Individual, sqlx::Error> {
        // let individual: Individual<chrono::Utc> = sqlx::query_as("SELECT * FROM test where id = ?")
        let individual: Individual = sqlx::query_as("SELECT * FROM test where id = ?")
            .bind(id)
            .fetch_one(&self.pool).await?;
        Ok(individual)
    }
}

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:member@172.18.0.2/test").await?;

    let conn = Conn { pool };

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let me = conn.individual_by_id(1).await?;
    println!("{:?}", me);

    Ok(())
}