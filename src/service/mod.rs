use crate::error::ApiError; 

pub mod task;
pub mod group;
pub mod user;
pub mod member;

pub trait Service {
    type View;
    type Draft;
    type Id;

    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::View>, ApiError>;
    async fn get_all(&self) -> Result<Vec<Self::View>, ApiError>;
    async fn create(&self, draft: Self::Draft) -> Result<Self::View, ApiError>;
}
