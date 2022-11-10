// Rust webserver using rocket
// By Colby Reinhart
// 10-26-2022

#[macro_use] extern crate rocket;

use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use rocket::http::Header;
use rocket::response::Responder;

//
// Rocket boilerplate
//

#[launch]
fn rocket() -> _
{
    rocket::build().mount("/", routes![
		index,
		get_static,
		get_favicon,
		get_page,
		get_downloads,
		get_media_file
	])
}

// Derive a responder to set correct content-disposition for downloads
#[derive(Responder)]
struct DownloadResponder<T>
{
    inner: T,
    content_disposition: Header<'static>,
}
impl<'r, 'o: 'r, T: Responder<'r, 'o>> DownloadResponder<T>
{
    fn new(inner: T, header_value: String) -> Self
	{
        DownloadResponder
		{
            inner,
            content_disposition: Header::new("content-disposition", header_value),
        }
    }
}

//
// Route favicon
//

#[get("/favicon.ico")]
async fn get_favicon() -> Option<NamedFile>
{
	NamedFile::open(Path::new("favicon.ico")).await.ok()	
}

//
// Route static folder
//

#[get("/static/<file..>")]
async fn get_static(file: PathBuf) -> Option<NamedFile>
{
	NamedFile::open(Path::new("static/").join(file)).await.ok()	
}

//
// Route main website html
//

#[get("/")]
async fn index() -> Option<NamedFile>
{
    NamedFile::open(Path::new("templates/homepage.html")).await.ok()
}

#[get("/<page>")]
async fn get_page(page: &str) -> Option<NamedFile>
{
	let mut page_path: String = "templates/".to_owned();
	page_path.push_str(page);
	page_path.push_str(".html");
	NamedFile::open(Path::new(&page_path)).await.ok()
}

//
// Route downloads
//

#[get("/downloads/<file..>")]
async fn get_downloads(file: PathBuf) -> DownloadResponder<Option<NamedFile>>
{
	DownloadResponder::new(NamedFile::open(Path::new("local-files/downloads/").join(file)).await.ok(), "attachment".to_string())
}

//
// Route media files
//

#[get("/files/<file..>")]
async fn get_media_file(file: PathBuf) -> Option<NamedFile>
{
	NamedFile::open(Path::new("local-files/media-files/").join(file)).await.ok()
}
