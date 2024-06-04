

    use std::env;

    use sqlx::{MySql, MySqlPool, Pool};

    use crate::error::error::ErrorMessage;

    pub async fn connect() -> Result<MySqlPool, ErrorMessage> {
        if let Err(e) = dotenv::dotenv() {
            return Err(ErrorMessage::new(&format!(
                "Failed to read .env file: {}",
                e
            )));
        }
    
        let database_url =
            env::var("DATABASE_URL").map_err(|_| ErrorMessage::new("DATABASE_URL is not set"))?;
    
        let pool_result = MySqlPool::connect(&database_url)
            .await
            .map_err(|e| ErrorMessage::new(&format!("Database connection error: {}", e)))?;
    
        Ok(pool_result)
    }


    pub async fn pool() -> Pool<MySql> {

        let p: Pool<MySql> = connect().await.unwrap();

        p
    }
