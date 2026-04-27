use super::Service;
use crate::repository::{
    group_repository::GroupRepo,
    user_repository::UserRepo,
    member_repository::MemberRepo
};
use crate::models::member::{MemberDraft, MemberView, Member};
use crate::error::ApiError;
use surrealdb::types::RecordId;

#[derive(Clone)]
pub struct MemberService<M, U, G>
where
    M: MemberRepo,
    U: UserRepo,
    G: GroupRepo
{
    pub member_repository: M,
    pub user_repository: U,
    pub group_repository: G

}

impl<M, U, G> MemberService<M, U, G>
where
    M: MemberRepo,
    U: UserRepo,
    G: GroupRepo
{
    pub fn new(member_repository: M, user_repository: U, group_repository: G) -> Self {
        Self {
            member_repository,
            user_repository,
            group_repository
        }
    }
}

impl<M, U, G> Service for MemberService<M, U, G>
where
    M: MemberRepo,
    U: UserRepo,
    G: GroupRepo
{
    type Id = RecordId;
    type View = MemberView;
    type Draft = MemberDraft;

    async fn get_all(&self) -> Result<Vec<Self::View>, ApiError> {
        self.member_repository
            .list()
            .await?
            .into_iter()
            .map(MemberView::try_from)
            .collect()
    }

    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::View>, ApiError> {
        self.member_repository
            .get(id)
            .await?
            .map(MemberView::try_from)
            .transpose()
    }

    async fn create(&self, draft: Self::Draft) -> Result<Self::View, ApiError> {
        let member = Member::from(draft);
        let member = self.member_repository.create(member).await?;

        MemberView::try_from(member)
    }
}
