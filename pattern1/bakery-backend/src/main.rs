// main.rs

use chef::ActiveModel;
use futures::executor::block_on;
use sea_orm::*;
mod entities;
use entities::{prelude::*, *};

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.
const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5433/bakeries_db";

async fn run() -> Result<(), DbErr> {
    let db: DatabaseConnection = Database::connect(DATABASE_URL).await?;

    //insert
    let happy_bakery: bakery::ActiveModel = bakery::ActiveModel {
        name: ActiveValue::Set("Happy Bakery".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let res: InsertResult<bakery::ActiveModel> = Bakery::insert(happy_bakery).exec(&db).await?;

    //update
    let sad_bakery: bakery::ActiveModel = bakery::ActiveModel {
        id: ActiveValue::Set(res.last_insert_id),
        name: ActiveValue::Set("Sad Bakery".to_owned()),
        profit_margin: ActiveValue::NotSet,
    };
    sad_bakery.update(&db).await?;

    //insert of chef
    let john: ActiveModel = chef::ActiveModel {
        name: ActiveValue::Set("John".to_owned()),
        bakery_id: ActiveValue::Set(res.last_insert_id),
        ..Default::default()
    };
    Chef::insert(john).exec(&db).await?;

    //search
    // Finding all
    let bakeries: Vec<bakery::Model> = Bakery::find().all(&db).await?;
    println!("{:?}", bakeries);
    // Finding bt id
    let bakery_id: Option<bakery::Model> = Bakery::find_by_id(1).one(&db).await?;
    println!("{:?}", bakery_id);
    // Finding by arbitrary column with `filter()`
    let bakery_filter: Option<bakery::Model> = Bakery::find()
        .filter(bakery::Column::Name.eq("Sad Bakery"))
        .one(&db)
        .await?;
    println!("{:?}", bakery_filter);

    //delete
    let del_john = chef::ActiveModel {
        id: ActiveValue::Set(1), // The primary key
        ..Default::default()
    };
    del_john.delete(&db).await?;

    let del_sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(1), // The primary key
        ..Default::default()
    };
    del_sad_bakery.delete(&db).await?;

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
