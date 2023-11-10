// proyecto/src/backend.rs
use sqlx::{ConnectOptions, Executor};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::postgres::PgConnectOptions;
use anyhow::{Result, anyhow};
use warp::{Rejection, Reply};

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseConnection {
    choice: u32,
    host: Option<String>,
    username: Option<String>,
    password: Option<String>,
    database: Option<String>,
}

pub async fn handle_connect(db_connection: DatabaseConnection) -> Result<impl Reply, Rejection> {
    let result = run_database_program(&db_connection).await?;
    Ok(warp::reply::html(result))
}

async fn run_database_program(db_connection: &DatabaseConnection) -> Result<String> {
    match db_connection.choice {
        1 => {
            let options = SqliteConnectOptions::new()
                .filename(db_connection.database.as_ref().ok_or(anyhow!("Database name is required"))?.as_str())
                .create_if_missing(true);
            
            connect_and_execute(options).await
        }
        2 => {
            let host = db_connection.host.as_ref().ok_or(anyhow!("Host is required"))?;
            let username = db_connection.username.as_ref().ok_or(anyhow!("Username is required"))?;
            let password = db_connection.password.as_ref().ok_or(anyhow!("Password is required"))?;
            let database = db_connection.database.as_ref().ok_or(anyhow!("Database name is required"))?;

            let options = MySqlConnectOptions::new()
                .host(host)
                .username(username)
                .password(password)
                .database(database);
            
            connect_and_execute(options).await
        }
        3 => {
            let host = db_connection.host.as_ref().ok_or(anyhow!("Host is required"))?;
            let username = db_connection.username.as_ref().ok_or(anyhow!("Username is required"))?;
            let password = db_connection.password.as_ref().ok_or(anyhow!("Password is required"))?;
            let database = db_connection.database.as_ref().ok_or(anyhow!("Database name is required"))?;

            let options = PgConnectOptions::new()
                .host(host)
                .username(username)
                .password(password)
                .database(database);
            
            connect_and_execute(options).await
        }
        _ => Err(anyhow!("Invalid choice")),
    }
}

async fn connect_and_execute<O>(options: O) -> Result<String>
where
    O: ConnectOptions,
{
    let pool = options.connect().await?;
    // Operaciones en la base de datos
    // ...
    Ok("Operación realizada con éxito. ¡Conexión establecida!".to_owned())
}
