use crate::prelude::*;
use rocket::http::{Status, Cookies, Cookie};
use rocket::request::{FromForm, FromRequest, Outcome, Request};

#[derive(Debug)]
pub struct Session<'a> {
    pub cookies: Cookies<'a>,
    pub id: u64,
    pub email: String,
    pub auth_key: String,
    pub time_stamp: u32,
}


impl<'a, 'r> FromRequest<'a, 'r> for Session<'a> {
    type Error = Error;
    fn from_request(request: &'a Request<'r>) -> Outcome<Session<'a>, Self::Error> {    
        let fields = get_fields(&mut request.cookies());

        if are_all_some(&fields) {
            if let Some(session) = session(request.cookies(), fields) {
                Outcome::Success(session)
            } else {
                Outcome::Failure((
                    Status::Unauthorized,
                    Error {
                        message: "Request cookies fields could't be parsed to its proper types."
                            .into(),
                        kind: ErrorKind::ClientSessionError,
                    },
                ))
            }
        } else {
            Outcome::Failure((
                Status::Unauthorized,
                Error {
                    message: "Request cookies didn't have the requeired fields.".into(),
                    kind: ErrorKind::ClientSessionError,
                },
            ))
        }
    }
}

type OptionCookie<'a> = Option<rocket::http::Cookie<'a>>;

fn get_fields<'a>(cookies: &mut Cookies) -> [OptionCookie<'a>; 4]{
    let id = cookies.get_private("id");
    let email = cookies.get_private("email");
    let auth_key = cookies.get_private("auth_key");
    let time_stamp = cookies.get_private("time_stamp");
    [id, email, auth_key, time_stamp]
}


fn are_all_some(array: &[OptionCookie]) -> bool {
    array
        .iter()
        .map(|x| x.is_some())
        .fold(true, |x, y| x && y)
}

fn session<'a>(cookies: Cookies<'a>, [id, email, auth_key, time_stamp]: [OptionCookie; 4]) -> Option<Session<'a>> {
    let result = id?.value().parse();
    let id;
    if let Ok(id_) = result {
        id = id_;
    } else {
        return None;
    }
    let result = email?.value().parse();
    let email;
    if let Ok(email_) = result {
        email = email_;
    } else {
        return None;
    }
    let result = auth_key?.value().parse();
    let auth_key;
    if let Ok(auth_key_) = result {
        auth_key = auth_key_;
    } else {
        return None;
    }
    let result = time_stamp?.value().parse();
    let time_stamp;
    if let Ok(time_stamp_) = result {
        time_stamp = time_stamp_;
    } else {
        return None;
    }
    Some(Session {
        cookies,
        id,
        email,
        auth_key,
        time_stamp,
    })
}



use std::time::{SystemTime, UNIX_EPOCH};
fn now() -> u128 {
    use crate::error::SetErrorMessage;
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .msg("Error computing SystemTime")
        .unwrap_or(Duration::from_secs(0))
        .as_millis()
}
