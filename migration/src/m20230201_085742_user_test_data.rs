use entity::user;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ActiveModelTrait, EntityTrait, Set},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        user::ActiveModel {
            firstname: Set("Mitchell".to_owned()),
            lastname: Set("Dederer".to_owned()),
            email: Set("mitchelldederer@gmail.com".to_owned()),
            secret: Set("Wouldn't you like to know!".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;
        user::ActiveModel {
            firstname: Set("Eden".to_owned()),
            lastname: Set("Lindley".to_owned()),
            email: Set("eden.lindley@hotmail.com".to_owned()),
            secret: Set("Wouldn't you like to know!".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;
        user::ActiveModel {
            firstname: Set("Beauty".to_owned()),
            lastname: Set("Dederer".to_owned()),
            email: Set("booty@catmail.meow".to_owned()),
            secret: Set("Wouldn't you like to know!".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;
        user::ActiveModel {
            firstname: Set("Minkie".to_owned()),
            lastname: Set("Lindley".to_owned()),
            email: Set("imhungry@catmail.meow".to_owned()),
            secret: Set("Wouldn't you like to know!".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;
        user::ActiveModel {
            firstname: Set("Annabel".to_owned()),
            lastname: Set("Dederer".to_owned()),
            email: Set("annabel@gmailmaybe.com".to_owned()),
            secret: Set("Wouldn't you like to know!".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        user::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}
