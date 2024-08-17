use ::entity::{post, post::Entity as Post};
use sea_orm::*;

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5433/test_db";

#[tokio::main]
async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    //insert
    let post_model = post::ActiveModel {
        content: Set("Hello World from SeaORM".to_owned()),
        ..Default::default()
    };
    let post = Post::insert(post_model).exec(&db).await?;
    println!("Inserted post: {:?}", post);
    Ok(())
}

fn main() {
    let _ = run();
}
