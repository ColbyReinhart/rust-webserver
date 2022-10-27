// Rust webserver using rocket
// By Colby Reinhart
// 10-26-2022

#[macro_use] extern crate rocket;

use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;

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
		get_nimbus
	])
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
// Route nimbus cloud
//
#[get("/nimbus")]
async fn get_nimbus() -> Option<NamedFile>
{
	NamedFile::open(Path::new("templates/nimbus.html")).await.ok()
}
