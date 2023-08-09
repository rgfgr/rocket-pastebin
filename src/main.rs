#[macro_use] extern crate rocket;

mod paste_id;

use paste_id::PasteId;

use std::path::Path;
use rocket::tokio::fs::File;

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, retrieve])
}
