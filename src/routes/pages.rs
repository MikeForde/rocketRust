use rocket::State;
use rocket_dyn_templates::{context, Template};

use crate::db::{load_ips, MySqlPool};

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/ipsview?<uuid>")]
pub async fn ips_view(uuid: String, pool: &State<MySqlPool>) -> Option<Template> {
    let ips = load_ips(&uuid, pool.inner())
        .await
        .expect("Failed to load IPS");

    ips.map(|model| {
        Template::render(
            "ips",
            context! {
                uuid: uuid,
                ips: model,
            },
        )
    })
}
