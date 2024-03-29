use std::io::Cursor;
use std::process::{Command, Stdio};

use rocket::response::Response;
use rocket::http::{ContentType, Status};

/// Posílá informace z harmonogramové tabulky
#[get("/harmonogram")]
pub fn harmonogram<'a>() -> Response<'a> {
	if let (Ok(prirodovedci), Ok(humanities), Ok(praktici)) =
		(String::from_utf8(Command::new("curl").arg("http://gsx2json.com/api?id=1PKzXl2buNNovjpGp6jK_YcVYZ-oEv8UmCSfNKvjtJX8&sheet=2&columns=false").stdout(Stdio::piped()).output().unwrap().stdout)
		,String::from_utf8(Command::new("curl").arg("http://gsx2json.com/api?id=1PKzXl2buNNovjpGp6jK_YcVYZ-oEv8UmCSfNKvjtJX8&sheet=3&columns=false").stdout(Stdio::piped()).output().unwrap().stdout)
		,String::from_utf8(Command::new("curl").arg("http://gsx2json.com/api?id=1PKzXl2buNNovjpGp6jK_YcVYZ-oEv8UmCSfNKvjtJX8&sheet=4&columns=false").stdout(Stdio::piped()).output().unwrap().stdout))
	{
		Response::build()
			.header(ContentType::JSON)
			.sized_body(Cursor::new(format!("{{ \"prirodovedci\": {}, \"humanities\": {}, \"praktici\": {} }}", prirodovedci, humanities, praktici)))
			.finalize()
	} else {
		Response::build()
			.status(Status::InternalServerError)
			.sized_body(Cursor::new("Internal Server Error"))
			.finalize()
	}
}
