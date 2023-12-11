use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    UserName,
    DisplayName,
}

#[derive(DeriveIden)]
enum Credential {
    Table,
    Id, // is user id
    Email,
    Passhash,
}

#[derive(DeriveIden)]
enum Session {
    Table,
    Id,
    UserId,
    Token,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum Service {
    Table,
    Id,
    Name,
    Domain,
    Description,
    OwnerId,
}

fn id_def<'a>(id: &'a mut ColumnDef) -> &'a mut ColumnDef {
    id.uuid().not_null().primary_key()
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Users
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(id_def(&mut ColumnDef::new(User::Id)))
                    .col(
                        ColumnDef::new(User::UserName)
                            .string_len(64)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::DisplayName).string_len(64).not_null())
                    .to_owned(),
            )
            .await?;

        // Credentials
        manager
            .create_table(
                Table::create()
                    .table(Credential::Table)
                    .if_not_exists()
                    .col(id_def(&mut ColumnDef::new(Credential::Id)))
                    .col(ColumnDef::new(Credential::Email).string_len(256).not_null())
                    .col(
                        ColumnDef::new(Credential::Passhash)
                            .string_len(256)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_credential_id")
                            .from(Credential::Table, Credential::Id)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Sessions
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(id_def(&mut ColumnDef::new(Session::Id)))
                    .col(
                        ColumnDef::new(Session::UserId)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Session::Token).string_len(256).not_null())
                    .col(ColumnDef::new(Session::ExpiresAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Service::Table)
                    .if_not_exists()
                    .col(id_def(&mut ColumnDef::new(Service::Id)))
                    .col(ColumnDef::new(Service::Name).string_len(64).not_null())
                    .col(ColumnDef::new(Service::Domain).string_len(64).not_null())
                    .col(ColumnDef::new(Service::Description).string_len(256))
                    .col(
                        ColumnDef::new(Service::OwnerId)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_service_owner_id")
                            .from(Service::Table, Service::OwnerId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        unimplemented!();
    }
}
