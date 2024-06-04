

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
    use actix_web::web;
    use actix_web::App;

    use actix_web::test;
    use rest_api::db::pool;
    use rest_api::route::delete_note_handler;
    use rest_api::route::notes_list_handler;
    use rest_api::route::single_note_handler;
    use rest_api::state::AppState;
    use crate::rust_rest::index;
    
    #[actix_web::test]
    async fn api() {
        let app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn all_notes() {

        let db_pool = pool().await;
        
        let app = test::init_service(
            App::new()
            .app_data(web::Data::new(AppState{db: db_pool.clone()}))
            .service(notes_list_handler))
            .await;
        let req = test::TestRequest::get().uri("/notes").param("limit", "5").param("offset", "0").to_request();
        let resp = test::call_service(&app, req).await;
        
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn single_note() {

        let db_pool = pool().await;
        
        let app = test::init_service(
            App::new()
            .app_data(web::Data::new(AppState{db: db_pool.clone()}))
            .service(single_note_handler))
            .await;
        let req = test::TestRequest::get().uri("/notes/1").to_request();
        let resp = test::call_service(&app, req).await;
        
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn delete_note() {

        let db_pool = pool().await;
        
        let app = test::init_service(
            App::new()
            .app_data(web::Data::new(AppState{db: db_pool.clone()}))
            .service(delete_note_handler))
            .await;
        let req = test::TestRequest::delete().uri("/notes/6").to_request();
        let resp = test::call_service(&app, req).await;
        
        assert!(resp.status().is_success());
    }


}


}