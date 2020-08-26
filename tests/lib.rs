#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use rust_at_one::{
        documents::post::Post, handlers::configure_post_routes, mongo::Mongo, AppState,
    };

    #[actix_rt::test]
    async fn test_index_get() {
        // tip: set up stack.yml (docker) for running a local mongo server for integration testing
        // todo: read from env
        let cs = "mongodb://root:example@localhost:27017/";

        let mongo = Mongo::initialize(cs).await.unwrap();
        let app_state = AppState::new(&mongo).wrap();
        let mut app =
            test::init_service(App::new().data(app_state).configure(configure_post_routes)).await;

        let post_one = Post {
            name: Some("Smooth".to_string()),
            id: None,
            author: Some("Kalle Lind".to_string()),
        };

        let mongo = Mongo::initialize(cs).await.unwrap();
        let res = mongo
            .main_db
            .collection("post")
            .insert_one(post_one.into(), None)
            .await
            .unwrap();
        let id = res
            .inserted_id
            .to_string()
            .replace("ObjectId(\"", "")
            .replace("\")", "");

        let req = test::TestRequest::get()
            .uri(format!("/api/posts/{}", id).as_str())
            .to_request();
        let resp: Post = test::read_response_json(&mut app, req).await;

        assert_eq!(resp.id.unwrap().to_string(), id);
    }
}
