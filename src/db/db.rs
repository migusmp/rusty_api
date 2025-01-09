use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use sqlx::PgPool;
use std::env;

// Función para obtener el pool de conexiones a la base de datos
pub async fn get_db_pool() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    // let pool = PgPool::connect(&database_url).await?;
    let pool = PgPoolOptions::new()
        .max_connections(350)
        .connect(&database_url)
        .await?;
    Ok(pool)
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
    pool: &PgPool,
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
        .execute(pool) // Ejecutamos sin transacción
        .await?;

    Ok(())
}

pub async fn delete_all_users(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Ejecutamos la consulta para borrar todos los datos de la tabla
    sqlx::query("DELETE FROM users").execute(pool).await?;

    Ok(())
}
