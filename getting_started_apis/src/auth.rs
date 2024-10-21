use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

pub struct BasicAuth { //this is a form of a class in python. It's a way to define the datatype in python.
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    // this is an example of a @staticmethod in a python class
    fn from_authorization_header(header: &str) -> Option<BasicAuth> { 
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
            return None;
        };

        if split[0] != "Basic" { // performing basic authorization
            return None;
        };

        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth> {
        let decoded = base64::decode(base64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":").collect::<Vec<_>>();

        // if the username and password are present
        if split.len() != 2 {
            return None;
        };

        let (username, password) = (split[0].to_string(), split[1].to_string());
        Some(BasicAuth {
            username,
            password
        })
    }
}

//Using request guard in rocket to call the BasicAuth
/// ``` request guards are used to preprocess incoming requests before they are handled by routes
/// // They allow you to perform checks or extract data from the request, and then decide whether
/// // to proceed with handling the request or return an error response.
///
/// // Request guards are implemented using traits that define
/// // how the data is extracted and processed from the request.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorization_header(auth_header) {
                return Outcome::Success(auth)
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}