mod models;

use crate::models::Person;
use rocket::form::{Contextual, Form};
use rocket::fs::{relative, FileServer, Options};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::{get, launch, post, routes, uri};
use rocket_dyn_templates::{context, Template};

#[launch]
fn rocket() -> _ {
    rocket::build()
        // add templating engine
        .attach(Template::fairing())
        // serve content from the "public" directory
        .mount(
            "/public",
            FileServer::new(
                relative!("/public"),
                Options::Missing | Options::NormalizeDirs,
            ),
        )
        // register routes
        .mount("/", routes![root, create, hello])
}

#[get("/")]
async fn root() -> Template {
    Template::render("root", context! { message: "Max är Bäst!"})
}

#[get("/hi?<name>")]
async fn hello(name: String, flash: Option<FlashMessage<'_>>) -> Template {
    let message = flash.map_or_else(|| String::default(), |msg| msg.message().to_string());
    Template::render("hello", context! {name, message})
}

#[post("/", data = "<form>")]
async fn create(form: Form<Contextual<'_, Person>>) -> Result<Flash<Redirect>, Template> {
    if let Some(ref person) = form.value {
        let name = format!("{} {}", person.first_name, person.last_name);
        let message = Flash::success(Redirect::to(uri!(hello(name))), "It Worked!");
        return Ok(message);
    }

    let error_messages: Vec<String> = form
        .context
        .errors()
        .map(|error| {
            let name = error.name.as_ref().unwrap().to_string();
            let description = error.to_string();
            format!("{}: {}", name, description)
        })
        .collect();

    Err(Template::render(
        "root",
        context! {
            first_name: form.context.field_value("first_name"),
            last_name: form.context.field_value("last_name"),
            first_name_error: form.context.field_errors("first_name").count() > 0,
            last_name_error: form.context.field_errors("last_name").count() > 0,
            errors: error_messages
        },
    ))
}