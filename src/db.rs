use std::env;

use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
};

use tokio::sync::OnceCell;

static POOL: OnceCell<Pool<AsyncPgConnection>> = OnceCell::const_new();

async fn build_connection_pool() -> Pool<AsyncPgConnection> {
    let connection_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(connection_url);
    Pool::builder().build(manager).await.unwrap()
}

pub async fn get_connection_pool() -> &'static Pool<AsyncPgConnection> {
    POOL.get_or_init(build_connection_pool).await
}

/// Get a database connection with automatic error handling
/// Example usage:
/// ```ignore
/// let conn = get_db!();
/// backend_template::schema::your_table::table.find(id).first::<YourTableModel>(&mut conn).await?;
/// ```
#[macro_export]
macro_rules! get_db {
    () => {
        $crate::db::get_connection_pool()
            .await
            .get()
            .await
            .map_err(|e| {
                tracing::error!("Unable to get database connection: {}", e);
                $crate::error::ErrorResponse::internal_server_error("Database connection failed")
            })?
    };
}
