use crate::{post, post::Entity as Post};
use sea_orm::*;

pub struct PostService;

impl PostService {
    pub async fn create_post(
        db: &DbConn,
        data: post::Model,
    ) -> Result<post::ActiveModel, DbErr> {
        post::ActiveModel {
            id: Set(data.id.to_owned()),
            title: Set(data.title.to_owned()),
            text: Set(data.text.to_owned()),
            // ..Default::default() # fix: To set default values(clippy)
        }
        .save(db)
        .await
    }

    pub async fn update_post_by_id(
        db: &DbConn,
        id: i32,
        data: post::Model,
    ) -> Result<post::Model, DbErr> {
        let post: post::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post::ActiveModel {
            id: post.id,
            title: Set(data.title.to_owned()),
            text: Set(data.text.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_post(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let post: post::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }
}
