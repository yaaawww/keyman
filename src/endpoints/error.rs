use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{
    options,
    catch,
    serde::json::{json, Value},
};
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        println!("Setting access control allow origin");
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "*",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
 
#[options("/<_..>")]
pub async fn all_options() {
    /* Intentionally left empty */
}

#[catch(404)]
pub async fn not_found() -> Value {
    json!("Not found!")
}
