use askama::Template;

#[derive(Template)]
#[template(path = "organization_ready.html.jinja")]
pub struct FirstWelcomeTemplate {
    pub admin_email: String,
    pub admin_password: String,
    pub plexo_url: String,
}
