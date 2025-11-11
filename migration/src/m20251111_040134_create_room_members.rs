use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RoomMembers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RoomMembers::RoomId)
                            .string_len(36)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RoomMembers::UserId)
                            .string_len(36)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RoomMembers::JoinedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_room_members")
                            .col(RoomMembers::RoomId)
                            .col(RoomMembers::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_room_members_room_id")
                            .from(RoomMembers::Table, RoomMembers::RoomId)
                            .to(Rooms::Table, Rooms::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_room_members_user_id")
                            .from(RoomMembers::Table, RoomMembers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_room_members_user_id")
                            .table(RoomMembers::Table)
                            .col(RoomMembers::UserId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RoomMembers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RoomMembers {
    Table,
    RoomId,
    UserId,
    JoinedAt,
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