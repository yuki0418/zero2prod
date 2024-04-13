use actix_web::{
    cookie::{time::Duration, Cookie},
    http::header::ContentType,
    web, HttpRequest, HttpResponse,
};
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use hmac::{Hmac, Mac};
use secrecy::ExposeSecret;
use std::fmt::Write;

use crate::startup::HmacSecret;

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: String,
    tag: String,
}

impl QueryParams {
    fn verify(self, secret: &HmacSecret) -> Result<String, anyhow::Error> {
        let tag = hex::decode(self.tag)?;
        let query_string = format!("error={}", urlencoding::Encoded::new(&self.error));

        let mut mac =
            Hmac::<sha2::Sha256>::new_from_slice(secret.0.expose_secret().as_bytes()).unwrap();
        mac.update(query_string.as_bytes());
        mac.verify_slice(&tag)?;

        Ok(self.error)
    }
}

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut error_html = String::new();
    for m in flash_messages.iter().filter(|m| m.level() == Level::Error) {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    HttpResponse::Ok()
        .content_type(ContentType::html())
        // .body(include_str!("login.html"))
        .body(format!(
            r#"
            <!DOCTYPE html>
            <html lang="ja-JP">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8" />
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Login</title>
            </head>
            <body>
                {error_html}
                <form action="/login" method="post">
                <label
                    >Username
                    <input type="text" placeholder="Enter Username" name="username" />
                </label>
                <label
                    >Password
                    <input type="password" placeholder="Enter Password" name="password" />
                </label>
                <button type="submit">Login</button>
                </form>
            </body>
            </html>
            "#
        ))
}
