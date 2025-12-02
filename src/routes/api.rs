use rocket::serde::json::Json;
use rocket::State;

use crate::db::{load_ips, MySqlPool};
use crate::models::IPSModel;

#[get("/ips/<package_uuid>")]
pub async fn get_ips(
    package_uuid: String,
    pool: &State<MySqlPool>,
) -> Option<Json<IPSModel>> {
    let ips = load_ips(&package_uuid, pool.inner())
        .await
        .expect("Failed to load IPS");

    ips.map(Json)
}

// You normally want DELETE, but keeping GET for compatibility
#[get("/ips/delbypra/<practitioner>")]
pub async fn delete_ips_by_practitioner(
    practitioner: String,
    pool: &State<MySqlPool>,
) -> Json<u64> {
    let result = sqlx::query(r#"DELETE FROM `ipsAlt` WHERE `patientPractitioner` = ?"#)
        .bind(&practitioner)
        .execute(pool.inner())
        .await
        .expect("Failed to delete from ipsAlt");

    let deleted = result.rows_affected();
    println!(
        "Deleted {} rows where patientPractitioner = {}",
        deleted, practitioner
    );

    Json(deleted)
}
