use std::sync::OnceLock;

use lettre::{
    transport::smtp::{authentication::Credentials, response::Response, Error},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::config::email::get_email_config;

static EMAIL_MAILER: OnceLock<AsyncSmtpTransport<Tokio1Executor>> = OnceLock::new();

fn get_email_mailer() -> AsyncSmtpTransport<Tokio1Executor> {
    EMAIL_MAILER
        .get_or_init(|| {
            let email_config = get_email_config();

            let credentials = Credentials::new(email_config.email, email_config.password);

            AsyncSmtpTransport::<Tokio1Executor>::relay(&email_config.smtp)
                .unwrap()
                .credentials(credentials)
                .build()
        })
        .clone()
}

pub async fn send_email(to: String, subject: String, content: String) -> Result<Response, Error> {
    let email = Message::builder()
        .from(format!("<{}>", get_email_config().email).parse().unwrap())
        .to(format!("<{}>", to).parse().unwrap())
        .subject(subject)
        .body(content)
        .unwrap();

    get_email_mailer().send(email).await
}
