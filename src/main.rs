use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{Error, Row};
use chrono;
use strum::EnumString;
use std::str::FromStr;

#[derive(sqlx::Type, EnumString)]
#[sqlx(rename = "department", rename_all = "snake_case")]
enum Department {
    Product,
    Design,
    Operation,
    Development,
    Magazine,
    NewMedia,
    HumanResources,
    Null,
}

#[derive(sqlx::Type)]
enum Sex {
    None,
    Male,
    Female,
    Other,
}

// #[derive(sqlx::FromRow)]
// struct Individual<Tz: chrono::TimeZone> {
struct Individual {
    id: i32,
    name: String,
    department: Department,
    student_id: String,
    sex: Sex,
    // grade: chrono::Date<Tz>,
    grade: String,
    major: String,
    class: String,
    // join_time: chrono::Date<Tz>,
    // leave_time: chrono::Date<Tz>,
    join_time: String,
    leave_time: String,
    description: String,
}

impl<'r> sqlx::FromRow<'r, MySqlRow> for Individual {
    fn from_row(row: &'r MySqlRow) -> Result<Self, Error> {
        let id = row.try_get("id")?;
        let name = row.try_get("name")?;
        let department_string: String = row.try_get("department")?;
        let department = match Department::from_str(&department_string) {
            Ok(t) => t,
            Err(e) => Department::Null,
        };
        let student_id = row.try_get("student_id")?;
        let sex = row.try_get("sex")?;
        let grade = row.try_get("grade")?;
        let major = row.try_get("major")?;
        let class = row.try_get("class")?;
        let join_time = row.try_get("join_time")?;
        let leave_time = row.try_get("leave_time")?;
        let description = row.try_get("description")?;
        Ok( Individual{
            id,
            name,
            department,
            student_id,
            sex,
            grade,
            major,
            class,
            join_time,
            leave_time,
            description,
        })
    }
}

struct Pool {
    pool: sqlx::Pool<sqlx::MySql>,
}

impl Pool {
    // async fn query_by_id<Tz: chrono::TimeZone>(&self, id: i32) -> Result<Individual<Tz>, sqlx::Error> {
    async fn query_by_id(&self, id: i32) -> Result<Individual, sqlx::Error> {
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

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: Individual = sqlx::query_as("SELECT * FROM test")
        //.bind("id")
        .fetch_one(&pool).await?;

    // println!("id = {}\n", row.id);
    // assert_eq!(row.0, 1);

    Ok(())
}