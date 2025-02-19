use sea_orm_migration::prelude::*;

use crate::m20250128_212125_create_user_table::User;

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
                    .col(ColumnDef::new(Post::Uuid)
                        .uuid()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(Post::Title)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(Post::Image)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(Post::IdUser)
                        .unsigned()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_post_user_id")
                        .from(Post::Table, Post::IdUser)
                        .to(User::Table, User::Id)
                    )
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
    Uuid,
    Title,
    Image,
    IdUser
}
