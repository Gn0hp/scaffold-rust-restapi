
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // config someting
    rocket_diesel_example::cmd::server::server().launch().await?;
    Ok(())
}