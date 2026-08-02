use oxicord_api_types::v10::user::ApiUser;
use oxicord_rest::RestBuilder;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let token = std::env::var("TOKEN").unwrap();
    let rest = RestBuilder::default().token(token).build();

    let user: ApiUser = rest.get("/users/@me").await.unwrap();

    println!("User: {user:#?}");
}
