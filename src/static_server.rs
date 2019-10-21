use rocket::response::NamedFile;

#[get("/")]
pub(crate) fn index() -> NamedFile {
    NamedFile::open("www/index.html").unwrap()
}

#[get("/style/<file>")]
pub(crate) fn styling(file: String) -> Option<NamedFile> {
    NamedFile::open(format!("style/{}", file)).ok()
}

#[get("/static/<file>")]
pub(crate) fn static_files(file: String) -> Option<NamedFile> {
    NamedFile::open(format!("static/{}", file)).ok()
}

#[get("/pkg/<file>")]
pub(crate) fn wasm_styling(file: String) -> Option<NamedFile> {
    NamedFile::open(format!("pkg/{}", file)).ok()
}

#[catch(404)]
pub(crate) fn not_found() -> NamedFile {
    NamedFile::open("www/404.html").unwrap()
}
