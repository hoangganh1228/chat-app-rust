use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .string_len(36)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Username)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Password)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .index(
                        Index::create()
                            .name("idx_email")
                            .table(Users::Table)
                            .col(Users::Email),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
    Password,
    CreatedAt,
}