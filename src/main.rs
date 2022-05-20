use sqlx::mysql::{MySqlPoolOptions};
use sqlx::{FromRow, Database, Pool, MySql, Error, Execute};

mod model {
    use sqlx::FromRow;

    #[derive(FromRow)]
    struct Cv {
        id: i32,
        dept_id: i32,
        state: i32,
        name: String,
        gender: i32,
        birth: String,
        // hometown: String,
        // nation: String,
        school: String,
        grade: String,
        class: String,
        dorm: String,
        phone: String,
        qq: String,
        mail: String,
        expr: String,
        intro: String,
        reason: String,
        cmt: String,
        time: String,       // chrono::Date
    }

    #[derive(FromRow)]
    struct Dept {
        id: i32,
        name: String,
        show: bool,
        mail, Text,
    }

    #[derive(FromRow)]
    struct Record {
        id: i32,
        dept_id: i32,
        uid: i32,
        op: String,
        time: String,
    }

    #[derive(FromRow)]
    struct User {
        id: i32,
        nick: String,
        openid: String,
        dept_id: i32,
        role: String,
    }
}

mod api {

    mod cv {
        use actix_web::{get, post};

        #[get("/getinfo")]
        async fn handle_get_cv(
            cur_usr: /* unknown */
        ) -> Result<impl Response> {
            Ok(web::Json(json!(
                {
                    "code": 0,
                    "cvList": cvList
                }
            )))
        }
    }
}

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

/*
struct CrudBase<T: FromRow, DB: Database> {
    model: T,
    pool: Pool<DB>,
}

impl<T, DB> CrudBase<T, DB>
where
    T: FromRow,
    DB: Database {

    fn new(model: T, pool: Pool<DB>) -> Self {
        CrudBase {
            model,
            pool
        }
    }

    async fn select_one_by(&self, )
}
*/

/*
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
        let individual: Individual = sqlx::query_as("SELECT * FROM test WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool).await?;
        Ok(individual)
    }
}
*/

#[derive(Debug)]
struct User {
    id: i32,
    username: String,
    email: String,
    password: String,
}

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut users = (0..).map(|i| User {
        id: i,
        username: format!("test_user_{}", i),
        email: format!("test-user-{}@example.com", i),
        password: format!("Test!User@Password#{}", i),
    });

    println!("{:?}", users.next());
    println!("{:?}", users.next());

    // Create a connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:member@172.18.0.2/test").await?;

    let mut query_builder: sqlx::QueryBuilder<MySql> = sqlx::QueryBuilder::new(
        "INSERT INTO users(id, username, email, password) "
    );

    query_builder.push_values(users.take(3), |mut b, user| {
        b.push_bind(user.id)
            .push_bind(user.username)
            .push_bind(user.email)
            .push_bind(user.password);
    });

    let mut query = query_builder.build();

    /*
    let sql = query.sql();
    let arguments = query.take_arguments().unwrap();

    println!("{}", sql);
    println!("{}", arguments.len());
    */

    //let conn = Conn::new(pool);

    //let individual = conn.individual_by_id(1).await?;

    //println!("{:?}", individual);

    Ok(())
}