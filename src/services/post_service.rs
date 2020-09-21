use super::{DocumentService};
use crate::{documents::Post, mongo::Mongo};
use async_trait::async_trait;
use mongodb::{bson::Document, Collection};

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

impl From<Post> for Option<Document> {
    fn from(post: Post) -> Self {
        Some(post.into())
    }
}

