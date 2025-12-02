use rocket::State;
use rocket_dyn_templates::{context, Template};

use crate::db::{load_ips, list_recent_packages, MySqlPool};

#[get("/")]
pub async fn index(pool: &State<MySqlPool>) -> Template {
    // Fetch, say, the 50 most recent packages; ignore errors & fall back to empty list
    let packages = list_recent_packages(pool.inner(), 50)
        .await
        .unwrap_or_default();

    Template::render("index", context! { packages: packages })
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
