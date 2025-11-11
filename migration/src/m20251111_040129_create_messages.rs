use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Messages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Messages::Id)
                            .string_len(36)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Messages::RoomId)
                            .string_len(36)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Messages::SenderId)
                            .string_len(36)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Messages::Content).text().not_null())
                    .col(
                        ColumnDef::new(Messages::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_messages_room_id")
                            .from(Messages::Table, Messages::RoomId)
                            .to(Rooms::Table, Rooms::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_messages_sender_id")
                            .from(Messages::Table, Messages::SenderId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_messages_room_id")
                            .table(Messages::Table)
                            .col(Messages::RoomId),
                    )
                    .index(
                        Index::create()
                            .name("idx_messages_sender_id")
                            .table(Messages::Table)
                            .col(Messages::SenderId),
                    )
                    .index(
                        Index::create()
                            .name("idx_messages_created_at")
                            .table(Messages::Table)
                            .col(Messages::CreatedAt),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Messages::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Messages {
    Table,
    Id,
    RoomId,
    SenderId,
    Content,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Rooms {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}