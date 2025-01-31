use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;

// Función para obtener el pool de conexiones a la base de datos
// pub async fn get_db_pool() -> Result<PgPool, sqlx::Error> {
//     dotenv().ok();
//
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
//
//     // let pool = PgPool::connect(&database_url).await?;
//     let pool = PgPoolOptions::new()
//         .max_connections(350)
//         .connect(&database_url)
//         .await?;
//     Ok(pool)
// }

pub async fn init_db_pool() -> Arc<PgPool> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let pool = PgPoolOptions::new()
        .max_connections(350)
        .idle_timeout(std::time::Duration::from_secs(10))
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    Arc::new(pool)
}

// Función para crear la tabla de usuarios si no existe
pub async fn create_users_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    let create_table_query = r#"
    CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        username VARCHAR NOT NULL,
        email VARCHAR UNIQUE NOT NULL,
        password VARCHAR NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );
    "#;

    sqlx::query(create_table_query).execute(pool).await?;

    Ok(())
}

// Función para insertar un nuevo usuario
// Guardamos al usuario en la BBDD.
pub async fn insert_user(
    username: &String,
    email: &String,
    pool: &Arc<PgPool>,
    hashed_pwd: &String,
) -> Result<(), Error> {
    let query = r#"
        INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)
    "#;

    sqlx::query(query)
        .bind(username)
        .bind(email)
        .bind(hashed_pwd)
        .execute(&**pool) // Ejecutamos sin transacción
        .await?;

    Ok(())
}

pub async fn update_user_image(
    user_id: i32,
    image_url: &String,
    pool: &Arc<PgPool>,
) -> Result<(), Error> {
    let query = r#"
        UPDATE users
        SET image = $1
        WHERE id = $2
    "#;

    sqlx::query(query)
        .bind(image_url)
        .bind(user_id)
        .execute(&**pool) // Ejecutamos sin transacción
        .await?;

    Ok(())
}

pub enum CheckResult {
    EXISTS,
    NONEXISTS,
    CONSULTERROR,
}

pub async fn check_username(username: String, pool: &Arc<PgPool>) -> CheckResult {
    let query = r#"
        SELECT username 
        FROM users 
        WHERE username = $1;
    "#;

    match sqlx::query(query).bind(&username).execute(&**pool).await {
        Ok(info) => {
            if info.rows_affected() > 0 {
                return CheckResult::EXISTS;
            }
            return CheckResult::NONEXISTS;
        }
        Err(_) => CheckResult::CONSULTERROR,
    }
}

pub async fn update_user_name(
    new_username: String,
    id: i32,
    pool: &Arc<PgPool>,
) -> Result<(), Error> {
    let query = r#"
        UPDATE users
        SET username = $1
        WHERE id = $2
    "#;

    sqlx::query(query)
        .bind(new_username)
        .bind(id)
        .execute(&**pool)
        .await?;

    Ok(())
}

pub async fn delete_all_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Ejecutamos la consulta para borrar todos los datos de la tabla
    sqlx::query("DELETE FROM users").execute(pool).await?;
    sqlx::query("DELETE FROM friend_requests")
        .execute(pool)
        .await?;
    sqlx::query("DELETE FROM friends").execute(pool).await?;

    Ok(())
}
