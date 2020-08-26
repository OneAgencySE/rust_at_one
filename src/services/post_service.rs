use super::{DocumentService, Dto, Query};
use crate::{documents::post::Post, mongo::Mongo};
use async_trait::async_trait;
use mongodb::Collection;

pub struct PostService {
    col: Collection,
}

#[async_trait]
impl DocumentService<Post> for PostService {
    type Query = Post;

    fn new(mongo: &Mongo) -> Self {
        PostService {
            col: mongo.main_db.collection("post"),
        }
    }

    fn name(&self) -> &str {
        "post"
    }

    fn collection(&self) -> &Collection {
        &self.col
    }
}

impl Dto for Post {
    fn set_id(&mut self, id: String) {
        self.id = Some(id)
    }
}

impl Query for Post {
    fn from_string_id(id: String) -> Self {
        Post {
            id: Some(id),
            name: None,
            author: None,
        }
    }
}
