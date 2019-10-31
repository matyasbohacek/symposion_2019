use rocket::response::NamedFile;
use std::path::{PathBuf, Path};

#[get("/")]
pub(crate) fn index() -> NamedFile {
    NamedFile::open("www/index.html").unwrap()
}

#[get("/style/<file..>")]
pub(crate) fn styling(file: PathBuf) -> Option<NamedFile> {
	NamedFile::open(Path::new("style/").join(file)).ok()
}

#[get("/static/<file..>")]
pub(crate) fn static_files(file: PathBuf) -> Option<NamedFile> {
	NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/pkg/<file..>", rank = 0)]
pub(crate) fn wasm_styling(file: PathBuf) -> Option<NamedFile> {
	NamedFile::open(Path::new("pkg/").join(file)).ok()
}

#[catch(404)]
pub(crate) fn not_found() -> NamedFile {
    NamedFile::open("www/404.html").unwrap()
}
