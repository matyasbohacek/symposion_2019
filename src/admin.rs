pub use admin::*;

mod admin{
    
    use rocket::FromRequest;

    pub struct AdminGuard;

    impl FromRequest for AdminGuard{
        type Error = String;
        fn from_request(){
            unimplemented!();
        }
    }

}
