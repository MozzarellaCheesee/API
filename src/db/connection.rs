use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;

pub type DbPool = Pool<AsyncPgConnection>;

pub async fn establish_connection(db_url: &str) -> DbPool {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    Pool::builder().build(config).await.expect("Failed to create pool")
}