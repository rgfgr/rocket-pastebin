#[macro_use] extern crate rocket;
mod paste_id;
use paste_id::PasteId;
use rocket::{
    http::uri::Absolute,
    data::{
        Data,
        ToByteUnit
    },
    tokio::fs::File,
};

const ID_LENGTH: usize = 3;
const HOST: Absolute<'static> = uri!("http://localhost:8000");

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}

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
    rocket::build().mount("/", routes![index, retrieve, upload])
}
