use actix_web::test::{self, init_service};
use actix_web::web::Data;
use actix_web::{
    http::{header, StatusCode},
    web, App,
};
use bytes::Bytes;
use mongodb::bson::{doc, oid::ObjectId, Document};
use rust_at_one::handlers::configure_routes;
use rust_at_one::mongo::Mongo;
use rust_at_one::{AppConfig, AppEnv, AppState};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[cfg(test)]
mod tests {
    use crate::{ReqVerb, TestService};
    use actix_web::http::StatusCode;
    use rstest::*;
    use rust_at_one::documents::Post;
    use uuid::Uuid;

    #[rstest(
        url,
        count,
        case("/api/posts?name=jibberIsh87", 0),
        case("/api/posts?name=One", 5), // collides should be 1
        case("/api/posts", 10), // Collides should be 5
        case("/api/posts?number=1&count=2", 2),
        case("/api/posts?number=2&count=2", 2) // Collides should be 1
    )]
    #[actix_rt::test]
    async fn post_get_many(url: &str, count: usize) {
        let mut service = TestService::init("post".to_string()).await;
        let id = Uuid::new_v4().to_string();
        service
            .insert(vec![
                Post {
                    id: None,
                    name: Some("One".to_string()),
                    author: Some(id.clone()),
                },
                Post {
                    id: None,
                    name: Some("Two".to_string()),
                    author: Some(id.clone()),
                },
                Post {
                    id: None,
                    name: Some("Three".to_string()),
                    author: Some(id.clone()),
                },
                Post {
                    id: None,
                    name: Some("Four".to_string()),
                    author: Some(id.clone()),
                },
                Post {
                    id: None,
                    name: Some("Five".to_string()),
                    author: Some(id.clone()),
                },
            ])
            .await;
        let r = ReqVerb::Get::<String>(url);
        let resp: (Vec<Post>, StatusCode) = service.make_req(r).await;
        service.clean_up().await;

        assert_eq!(resp.0.len(), count);
        assert_eq!(resp.1, 200);
    }
}

enum ReqVerb<'a, T> {
    Post(&'a str, T),
    Put(&'a str, T),
    Get(&'a str),
    Delete(&'a str),
}

struct TestService {
    mongo: Mongo,
    app_state: Data<AppState>,
    collection: String,
    clean_up: Option<Vec<ObjectId>>,
}

impl TestService {
    pub async fn init(collection: String) -> Self {
        let config = AppConfig::new(AppEnv::FromFile("test.env"));

        let mongo = Mongo::initialize(config.mongo_db_uri.as_str(), config.db_name.as_str())
            .await
            .unwrap();

        let app_state = AppState::new(&mongo).wrap();

        TestService {
            mongo,
            app_state,
            clean_up: None,
            collection,
        }
    }

    pub async fn insert<T>(&mut self, input: Vec<T>)
    where
        T: Into<Document> + Clone,
    {
        let d: Vec<Document> = input.iter().map(|x| x.clone().into()).collect();
        let result = self
            .mongo
            .main_db
            .collection(&self.collection)
            .insert_many(d, None)
            .await
            .unwrap();
        self.clean_up = Some(
            result
                .inserted_ids
                .iter()
                .map(|x| x.1.as_object_id().unwrap().clone())
                .collect(),
        );
    }

    pub async fn clean_up(&self) {
        if let Some(s) = &self.clean_up {
            self.mongo
                .main_db
                .collection(self.collection.as_str())
                .delete_many(doc! {"_id": {"$in": s}}, None)
                .await
                .unwrap();
        }
    }

    pub async fn make_req<In, Out>(&self, req_verb: ReqVerb<'_, In>) -> (Out, StatusCode)
    where
        In: Serialize + Into<web::Bytes>,
        Out: DeserializeOwned,
    {
        let mut app = init_service(
            App::new()
                .app_data(self.app_state.clone())
                .service(web::scope("/api").configure(configure_routes)),
        )
        .await;

        let req = match req_verb {
            ReqVerb::Post(p, v) => test::TestRequest::post().uri(p).set_payload(v),
            ReqVerb::Put(p, v) => test::TestRequest::put().uri(p).set_payload(v),
            ReqVerb::Get(p) => test::TestRequest::get().uri(p),
            ReqVerb::Delete(p) => test::TestRequest::delete().uri(p),
        };
        let req = req
            .header(header::CONTENT_TYPE, "application/json")
            .to_request();

        let response = test::call_service(&mut app, req).await;
        let status = response.status().clone();
        let m: Bytes = test::read_body(response).await;
        let model: Out = serde_json::from_str(std::str::from_utf8(&m).unwrap()).unwrap();

        (model, status)
    }
}
