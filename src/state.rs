use sqlx::Pool;
use sqlx::MySql;

pub struct AppState {
    pub db: Pool<MySql>
  }