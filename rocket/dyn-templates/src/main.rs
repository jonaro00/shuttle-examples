#[macro_use]
extern crate rocket;
// uncomment if you also wish to serve static resources
// use rocket::fs::{FileServer, relative};
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/", hello(name = "Your Name")))
}
#[get("/hello/<name>")]
pub fn hello(name: &str) -> Template {
    Template::render(
        "index",
        context! {
            title: "Hello",
            name: Some(name),
            items: vec!["Example", "List", "Of", "Five", "Items"],
        },
    )
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    // Note that shuttle does not include Rocket.toml
    // so merging config is the preferred way to modify any settings
    // that would otherwise be set in Rocket.toml

    let template_dir = "templates";
    let figment = rocket::Config::figment().merge(("template_dir", template_dir));
    let rocket = rocket::custom(figment)
        // If you also wish to serve static content, uncomment line below and corresponding 'use' on line 4
        // .mount("/", FileServer::from(relative!("templates")))
        .mount("/", routes![index, hello])
        .attach(Template::fairing());

    Ok(rocket.into())
}
