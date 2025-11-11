use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Rooms::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Rooms::Id)
                            .string_len(36)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Rooms::Name)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Rooms::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Rooms::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Rooms {
    Table,
    Id,
    Name,
    CreatedAt,
}