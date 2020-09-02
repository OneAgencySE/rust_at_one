use crate::mongo::to_object_id;
use mongodb::bson::{doc, Document};
use mongodb::options::UpdateModifications;
use serde::{Deserialize, Serialize};

/// Data model for Post
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    /// Mongodb id: _id
    pub id: Option<String>,
    /// Name of post
    pub name: Option<String>,
    /// Author name
    pub author: Option<String>,
}

impl From<Post> for UpdateModifications {
    fn from(p: Post) -> Self {
        let mut base_doc: Document = p.into();
        base_doc.remove("_id");
        let document = doc! {"$set": base_doc};
        UpdateModifications::Document(document)
    }
}

impl From<Post> for Document {
    fn from(p: Post) -> Self {
        let mut document = doc! {};

        if let Some(name) = p.name {
            document.insert("name", name);
        }

        if let Some(author) = p.author {
            document.insert("author", author);
        }

        if let Some(id) = p.id {
            if let Ok(obejct_id) = to_object_id(id.as_str()) {
                document.insert("_id", obejct_id);
            }
        }

        document
    }
}

impl From<Document> for Post {
    fn from(doc: Document) -> Self {
        let id = match doc.get_object_id("_id") {
            Ok(v) => Some(v.to_string()),
            Err(_) => None,
        };

        let name = doc
            .get_str("name")
            .map(|x| Some(x.to_string()))
            .unwrap_or(None);

        let author = doc
            .get_str("author")
            .map(|c| Some(c.to_string()))
            .unwrap_or(None);

        Post { id, name, author }
    }
}

/// DTO for updating and creating new Posts
#[derive(Deserialize, Debug, PartialEq)]
pub struct PostUpsert {
    name: Option<String>,
    author: Option<String>,
}

impl From<PostUpsert> for Post {
    fn from(p: PostUpsert) -> Self {
        Post {
            id: None,
            name: p.name,
            author: p.author,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn from_str_to_post_upsert() {
        let s = "{\"name\":\"value1\", \"author\":\"value2\"}";

        let e = PostUpsert {
            name: Some("value1".to_string()),
            author: Some("value2".to_string()),
        };
        let r: PostUpsert = serde_json::from_str(s).unwrap();
        assert_eq!(r, e);
    }

    #[test]
    fn from_str_to_post_upsert_only_name() {
        let i = "{\"name\":\"value1\"}";

        let e = PostUpsert {
            name: Some("value1".to_string()),
            author: None,
        };
        let r: PostUpsert = serde_json::from_str(i).unwrap();
        assert_eq!(r, e);
    }

    #[test]
    fn from_str_to_post() {
        let i = Post {
            id: Some("MyId".to_string()),
            name: Some("value1".to_string()),
            author: Some("value2".to_string()),
        };

        let e = "{\"id\":\"MyId\",\"name\":\"value1\",\"author\":\"value2\"}";

        let r = serde_json::to_string(&i).unwrap();
        assert_eq!(r, e);
    }
}
