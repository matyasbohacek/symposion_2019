
pub use login::*;

mod login {

    use rocket::data::FromDataSimple;
    use rocket::data::Outcome;
    use rocket::{Data, Outcome::*, Request};
    use rocket::http::{ContentType, Status};
    use std::io::Read;

    const LIMIT: u64 = 256; // input data limit

    pub struct Login {
        login: String,
        password: String,
    }

    impl FromDataSimple for Login {
        type Error = String;

        fn from_data(req: &Request, data: Data) -> Outcome<Self, String> {
            let person_ct = ContentType::new("application", "x-login");
            if req.content_type() != Some(&person_ct) {
                return Outcome::Forward(data);
            }

            let mut string = String::new();
            if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
                return Failure((Status::InternalServerError, format!("{:?}", e)));
            }

            let (login, password) = match string.find(':') {
                Some(i) => (string[..i].to_string(), &string[(i + 1)..]),
                None => return Failure((Status::UnprocessableEntity, "':'".into())),
            };

            Success(Login {
                login: login.to_string(),
                password: password.to_string(),
            })
        }
    }
}
