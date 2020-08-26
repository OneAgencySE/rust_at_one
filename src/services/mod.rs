pub mod post_service;
use super::Result;
use crate::{error::AppError, mongo::Mongo};
use async_trait::async_trait;
use futures_util::stream::StreamExt;
use mongodb::{
    bson::{doc, Document},
    options::{FindOptions, UpdateModifications},
    Collection,
};
use serde::{Deserialize, Serialize};
use Into;

/// Specify a dto object.
pub trait Dto {
    /// Helps set id returned during creation of a document, used with POST
    fn set_id(&mut self, id: String);
}

/// Standard query from single string input
/// This should generate a standard query object
///
/// TODO: Make is more general, Not only a single string
pub trait Query {
    fn from_string_id(id: String) -> Self;
}

/// This represents a service for a document
/// It will set up Get one/Get many/Put/Post/delete as a standard for a document
#[async_trait]
pub trait DocumentService<T>
where
    T: From<Document> + Into<Document> + Into<UpdateModifications> + Send + Sync + Clone + Dto,
    Self::Query: Into<Document> + Send + Clone + Serialize + Sync + Query,
{
    type Query;

    /// Instantiate the service, use the Mongo instance to
    /// set up the internal collection;
    fn new(mongo: &Mongo) -> Self;
    /// Name of the DTO in subject
    fn name(&self) -> &str;
    /// Get a reference to the internal collection
    fn collection(&self) -> &Collection;

    /// Get one T from the DB, this implementation uses the _id from Mongo
    async fn get_one<'a>(&self, query: Self::Query) -> Result<T>
    where
        T: 'a,
    {
        match self
            .collection()
            .find_one(query.clone().into(), None)
            .await?
        {
            Some(t) => Ok(t.into()),
            None => Err(not_found(self.name(), &query)?),
        }
    }

    // TODO: Pagination
    async fn get_many<'a>(&self, query: Self::Query, pagination: Pagination) -> Result<Vec<T>>
    where
        T: 'a,
        Self::Query: 'a,
    {
        let options = pagination.as_find_options();
        // TODO: use page object
        // TODO: Validate query
        let mut cursor = self
            .collection()
            .find(Some(query.into()), Some(options))
            .await?;

        let mut results: Vec<T> = Vec::new();
        while let Some(x) = cursor.next().await {
            results.push(x?.into())
        }

        Ok(results)
    }

    async fn delete<'a>(&self, query: Self::Query) -> Result<()>
    where
        T: 'a,
    {
        let result = self
            .collection()
            .delete_one(query.clone().into(), None)
            .await?;
        if result.deleted_count > 0 {
            Ok(())
        } else {
            Err(not_found(self.name(), &query)?)
        }
    }

    async fn post<'a>(&self, data: T) -> Result<T>
    where
        T: 'a,
    {
        let res = self
            .collection()
            .insert_one(data.clone().into(), None)
            .await?;

        let mut result = data;
        res.inserted_id.as_str();
        // TODO: Refactor, Use ObjectId instead of converting?
        let id = res
            .inserted_id
            .to_string()
            .replace("ObjectId(\"", "")
            .replace("\")", "");
        result.set_id(id);
        Ok(result)
    }

    async fn put<'a>(&self, query: Self::Query, data: T) -> Result<T>
    where
        T: 'a,
    {
        let result = self
            .collection()
            .update_one(
                query.clone().into(),
                data, //TODO: Cannot use into here but still works, why?
                None,
            )
            .await?;

        // Possible bug (TODO)
        // Okay to use the same query here if top Query is only using _id
        // We should be sure of the ID here
        if result.modified_count > 0 {
            self.get_one(query).await
        } else {
            Err(not_found(self.name(), &query)?)
        }
    }
}

fn not_found<T>(name: &str, query: &T) -> Result<AppError>
where
    T: Serialize,
{
    Ok(AppError::NotFound(format!(
        "A {} with given filter: '{}' not found",
        name,
        serde_json::to_string(&query).map_err(|x| AppError::InternalServerError(x.to_string()))?
    )))
}

const PAGE_COUNT: i64 = 10;
const PAGE_NUMBER: i64 = 0;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    number: Option<i64>,
    count: Option<i64>,
}

impl Pagination {
    fn as_find_options(&self) -> FindOptions {
        let mut options = FindOptions::default();
        options.limit = Some(PAGE_COUNT);
        options.skip = Some(PAGE_NUMBER);

        if let Some(n) = self.count {
            if n >= 0 {
                options.limit = Some(n);
            }
        }

        if let Some(n) = self.number {
            if n >= 0 {
                options.skip = Some(n * options.limit.unwrap());
            }
        }

        options
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn pagination(number: Option<i64>, count: Option<i64>) -> Pagination {
        Pagination { number, count }
    }

    fn find_options(skip: Option<i64>, limit: Option<i64>) -> FindOptions {
        let mut find_options = FindOptions::default();
        find_options.skip = skip;
        find_options.limit = limit;
        find_options
    }

    #[rstest(
        input,
        expected,
        case(pagination(None, None), find_options(Some(0), Some(10))),
        case(pagination(Some(0), None), find_options(Some(0), Some(10))),
        case(pagination(Some(1), None), find_options(Some(10), Some(10))),
        case(pagination(Some(0), Some(100)), find_options(Some(0), Some(100))),
        case(pagination(Some(1), Some(100)), find_options(Some(100), Some(100))),
        case(pagination(None, Some(10)), find_options(Some(0), Some(10)))
    )]
    fn pagination_as_find_options(input: Pagination, expected: FindOptions) {
        let r = input.as_find_options();
        assert_eq!(r.skip, expected.skip);
        assert_eq!(r.limit, expected.limit);
    }
}
