use crate::auth::models::session::Identity;
use crate::projects::models::responses::QueryProjectsForClientResponse;
use crate::repositories::projects::ProjectsRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

const MAX_LIMIT: i32 = 100;

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryProjectsByCustomerInput {
    identity: Identity,
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
    name: Option<String>,
    cursor: Option<String>,
    limit: i32,
}

impl QueryProjectsByCustomerInput {
    pub fn new(
        identity: Identity,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        name: Option<String>,
        cursor: Option<String>,
        limit: Option<i32>,
    ) -> Self {
        Self {
            identity,
            from,
            to,
            name,
            cursor,
            limit: limit.unwrap_or(MAX_LIMIT),
        }
    }
}

pub struct QueryProjectsByCustomer<P: ProjectsRepository> {
    projects_repository: Arc<P>,
}

impl<P> QueryProjectsByCustomer<P>
where
    P: ProjectsRepository,
{
    pub fn new(projects_repository: Arc<P>) -> Self {
        Self {
            projects_repository,
        }
    }
}

#[async_trait]
impl<R> UseCase<QueryProjectsByCustomerInput, QueryProjectsForClientResponse>
    for QueryProjectsByCustomer<R>
where
    R: ProjectsRepository,
{
    async fn execute(
        &self,
        input: QueryProjectsByCustomerInput,
    ) -> Result<QueryProjectsForClientResponse> {
        let response = self
            .projects_repository
            .query(
                input.identity.id,
                input.from,
                input.to,
                input.name,
                input.cursor,
                input.limit,
            )
            .await?;

        Ok(QueryProjectsForClientResponse {
            projects: response.data,
            cursor: response.cursor,
        })
    }
}
