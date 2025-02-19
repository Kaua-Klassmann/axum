use sea_orm_migration::prelude::*;

use crate::m20250128_212125_create_user_table::User;
use crate::m20250128_212714_create_post_table::Post;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Like::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Like::Id)
                            .unsigned()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Like::IdUser).unsigned().not_null())
                    .col(ColumnDef::new(Like::IdPost).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_like_user_id")
                            .from(Like::Table, Like::IdUser)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_like_post_uuid")
                            .from(Like::Table, Like::IdPost)
                            .to(Post::Table, Post::Uuid),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Like::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Like {
    Table,
    Id,
    IdUser,
    IdPost,
}
