use std::io::Cursor;
use std::process::{Command, Stdio};

use rocket::response::Response;
use rocket::http::{ContentType, Status};

/// Posílá informace z harmonogramové tabulky
#[get("/harmonogram", format = "application/json")]
pub fn harmonogram<'a>() -> Response<'a> {
	if let (Ok(ctvrtek), Ok(patek), Ok(sobota)) =
		(String::from_utf8(Command::new("curl").arg("http://gsx2json.com/api?id=12Q1jmsBpZh1LHSAcMwXIwWTZwKMzFoypw_fUrDbWJEQ&sheet=1").stdout(Stdio::piped()).output().unwrap().stdout)
		,String::from_utf8(Command::new("curl").arg("http://gsx2json.com/api?id=12Q1jmsBpZh1LHSAcMwXIwWTZwKMzFoypw_fUrDbWJEQ&sheet=2").stdout(Stdio::piped()).output().unwrap().stdout)
		,String::from_utf8(Command::new("curl").arg("http://gsx2json.com/api?id=12Q1jmsBpZh1LHSAcMwXIwWTZwKMzFoypw_fUrDbWJEQ&sheet=3").stdout(Stdio::piped()).output().unwrap().stdout))
	{
		Response::build()
			.header(ContentType::JSON)
			.sized_body(Cursor::new(format!("{{ \"ctvrtek\": {}, \"patek\": {}, \"sobota\": {} }}", ctvrtek, patek, sobota)))
			.finalize()
	} else {
		Response::build()
			.status(Status::InternalServerError)
			.sized_body(Cursor::new("Internal Server Error"))
			.finalize()
	}

}
