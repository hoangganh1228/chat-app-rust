pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20251111_040042_create_users;
mod m20251111_040123_create_rooms;
mod m20251111_040129_create_messages;
mod m20251111_040134_create_room_members;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251111_040042_create_users::Migration),
            Box::new(m20251111_040123_create_rooms::Migration),
            Box::new(m20251111_040129_create_messages::Migration),
            Box::new(m20251111_040134_create_room_members::Migration),
        ]
    }
}
