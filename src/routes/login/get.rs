use actix_web::{http::header::ContentType, web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: Option<String>,
}

pub async fn login_form(query: web::Query<QueryParams>) -> HttpResponse {
    let error_html = match query.0.error {
        None => "".into(),
        Some(error_message) => format!(
            "<p><i>{}</i></p>",
            htmlescape::encode_minimal(&error_message)
        ),
    };
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
