


use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000001_create_user::User;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_auto(Post::Id).not_null())
                    .col(string(Post::Title).not_null())
                    .col(uuid(Post::Uuid).unique_key())
                    .col(string(Post::Text).not_null())
                    .col(string(Post::Image))
                    .col(integer(Post::UserId).not_null())
                    .col(date_time(Post::CreatedAt).not_null())


                    .foreign_key(ForeignKey::create()
                    .name("fk-posts-users-id")
                    .from(Post::Table, Post::UserId)
                    .to(User::Table, User::Id))
                    
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Post {
    Table,
    Id,
    Title,
    Text,
    Uuid,
    Image,
    UserId , 
    CreatedAt
}
