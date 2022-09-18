use rocket::{http::{Status}, request::{FromRequest, Outcome}};
// From request
pub struct AuthStruct {
    pub username: String,
    pub password: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthStruct {
    type Error = ();
    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let header_auth = request.headers().get_one("Authorization");
        if let Some(_auth) = header_auth {
            if let Some(auth) = Self::from_header(_auth) {
                return Outcome::Success(auth);
            }
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}

// Basic username:password
impl AuthStruct {
    fn from_header(header: &str) -> Option<AuthStruct> {
        let split_vec = header.split_whitespace().collect::<Vec<_>>();
        if split_vec.len() != 2 {
            return None;
        }
        if split_vec[0] != "Basic" {
            return None;
        }
        // base64
        Self::from_base64(split_vec[1])
    }
    
    fn from_base64(base64_string: &str) -> Option<AuthStruct> {
        let decoded = base64::decode(base64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split_vec = decoded_str.split(":").collect::<Vec<_>>();
        if split_vec.len() != 2 {
            return None;
        }
        let (username, password) = (split_vec[0].to_string(), split_vec[1].to_string());
        Some(AuthStruct {
            username,
            password
        })
    }
}