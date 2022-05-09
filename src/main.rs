use sqlx::mysql::MySqlPoolOptions;
use chrono;

enum Department {
    Product,
    Design,
    Operation,
    Development,
    Magazine,
    NewMedia,
    HumanResources,
}

enum Sex {
    None,
    Male,
    Female,
    Other,
}

struct Individual<Tz: chrono::TimeZone> {
    id: i32,
    name: String,
    department: Department,
    student_id: String,
    sex: Sex,
    grade: chrono::Date<Tz>,
    major: String,
    class: String,
    join_time: chrono::Date<Tz>,
    leave_time: chrono::Date<Tz>,
    description: String,
}

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:member@172.18.0.2/test").await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i32,) = sqlx::query_as("SELECT ? FROM test")
        .bind("id")
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 1);

    Ok(())
}