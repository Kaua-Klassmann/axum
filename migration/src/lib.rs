pub use sea_orm_migration::prelude::*;

mod m20241224_171518_create_user_table;
mod m20241224_172251_create_post_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241224_171518_create_user_table::Migration),
            Box::new(m20241224_172251_create_post_table::Migration),
        ]
    }
}
