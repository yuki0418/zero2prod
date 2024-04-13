use crate::startup::HmacSecret;
use actix_web::cookie::Cookie;
use actix_web::error::InternalError;
use actix_web::{web, HttpResponse, ResponseError};
use hmac::{Hmac, Mac};
use reqwest::{header::LOCATION, StatusCode};
use secrecy::ExposeSecret;
use secrecy::Secret;
use sqlx::PgPool;

use crate::{
    authentication::{validate_credentials, AuthError, Credentials},
    routes::error_chain_fmt,
};

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed.")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong.")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for LoginError {
    fn error_response(&self) -> HttpResponse {
        let query_string = format!("error={}", urlencoding::Encoded::new(self.to_string()));
        let secret: &[u8] = todo!();
        let hamc_tag = {
            let mut mac = Hmac::<sha2::Sha256>::new_from_slice(secret).unwrap();
            mac.update(query_string.as_bytes());
            mac.finalize().into_bytes()
        };
        HttpResponse::build(self.status_code())
            .insert_header((LOCATION, format!("/login?{query_string}&tag={hamc_tag:x}")))
            .finish()
    }

    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            // StatusCode::SEE_OTHER
            LoginError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LoginError::AuthError(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

#[tracing::instrument(
  skip(form, pool, secret),
  fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn login(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    secret: web::Data<HmacSecret>,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };
    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));

    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/"))
                .finish())
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            let query_string = format!("error={}", urlencoding::Encoded::new(e.to_string()));
            let hmac_tag = {
                let mut mac =
                    Hmac::<sha2::Sha256>::new_from_slice(secret.0.expose_secret().as_bytes())
                        .unwrap();
                mac.update(query_string.as_bytes());
                mac.finalize().into_bytes()
            };
            let response = HttpResponse::SeeOther()
                .insert_header((LOCATION, "/login"))
                .cookie(Cookie::new("_flash", e.to_string()))
                .finish();
            Err(InternalError::from_response(e, response))
        }
    }
}
