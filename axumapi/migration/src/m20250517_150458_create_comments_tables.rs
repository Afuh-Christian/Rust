use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20220101_000001_create_table::User, m20250517_145754_create_post_tables::Post};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(pk_auto(Comment::Id).auto_increment().not_null())
                    .col(integer(Comment::UserId).not_null())
                    .col(integer(Comment::PostId).not_null())
                    .col(date_time(Comment::CreatedAt).not_null())
                    .col(string(Comment::Text).not_null())
                    .foreign_key(ForeignKey::create().name("fk-comments-posts-id").from(Comment::Table , Comment::PostId).to(Post::Table , Post::Id))
                    .foreign_key(ForeignKey::create().name("fk-comments-users-id").from(Comment::Table , Comment::UserId).to(User::Table , User::Id))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Comment {
    Table,
    Id,
    UserId,
    PostId,
    CreatedAt,
    Text
}
