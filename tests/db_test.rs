

#[cfg(test)]
pub mod rust_rest {
use rest_api::db;
use rest_api::route::index;
use sqlx::MySql;
use sqlx::Pool;


#[sqlx::test]
fn db_connection() {
   
   let conn: Pool<MySql> = db::connect().await.unwrap();
    
  // Test connection validity
  let result = conn.acquire().await;

  assert!(result.is_ok(), "Failed to acquire connection from pool");

  drop(result);
   
}

pub mod  base {
    use actix_web::App;

    use actix_web::test;
    use crate::rust_rest::index;
    
    #[actix_web::test]
    async fn api() {
        let app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn notes() {
        
        let app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::get().uri("/notes").param("limit", "5").param("offset", "0").to_request();
        let resp = test::call_service(&app, req).await;
        
        assert!(resp.status().is_success());
    }
}
}