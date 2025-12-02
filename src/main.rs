#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{Build, Rocket, State};
use std::env;

use chrono::NaiveDateTime;
use sqlx::{mysql::MySqlPoolOptions, FromRow, MySql, Pool};
use rocket_dyn_templates::{context, Template};

type MySqlPool = Pool<MySql>;

/// Row as stored in the `ipsAlt` table (flat, like your Sequelize IPSModel)
#[derive(Debug, FromRow)]
struct IPSRow {
    #[sqlx(rename = "id")]
    id: i64, // `id` is the PK (AUTO_INCREMENT) on ipsAlt

    #[sqlx(rename = "packageUUID")]
    package_uuid: String,

    #[sqlx(rename = "timeStamp")]
    time_stamp: NaiveDateTime,

    #[sqlx(rename = "patientName")]
    patient_name: String,

    #[sqlx(rename = "patientGiven")]
    patient_given: String,

    #[sqlx(rename = "patientDob")]
    patient_dob: NaiveDateTime,

    #[sqlx(rename = "patientGender")]
    patient_gender: Option<String>,

    #[sqlx(rename = "patientNation")]
    patient_nation: String,

    #[sqlx(rename = "patientPractitioner")]
    patient_practitioner: String,

    #[sqlx(rename = "patientOrganization")]
    patient_organization: Option<String>,

    #[sqlx(rename = "patientIdentifier")]
    patient_identifier: Option<String>,

    #[sqlx(rename = "patientIdentifier2")]
    patient_identifier2: Option<String>,
}

/// Row for child tables
#[derive(Debug, FromRow)]
struct MedicationRow {
    #[sqlx(rename = "med_name")]
    name: String,

    #[sqlx(rename = "med_date")]
    date: NaiveDateTime,

    #[sqlx(rename = "med_dosage")]
    dosage: String,

    // nullable in DB -> Option<String>
    #[sqlx(rename = "med_system")]
    system: Option<String>,

    #[sqlx(rename = "med_code")]
    code: Option<String>,

    #[sqlx(rename = "med_status")]
    status: Option<String>,
}

#[derive(Debug, FromRow)]
struct AllergyRow {
    #[sqlx(rename = "all_name")]
    name: String,

    #[sqlx(rename = "all_criticality")]
    criticality: Option<String>,

    #[sqlx(rename = "all_date")]
    date: NaiveDateTime,

    #[sqlx(rename = "all_system")]
    system: Option<String>,

    #[sqlx(rename = "all_code")]
    code: Option<String>,
}

#[derive(Debug, FromRow)]
struct ConditionRow {
    #[sqlx(rename = "con_name")]
    name: String,

    #[sqlx(rename = "con_date")]
    date: NaiveDateTime,

    #[sqlx(rename = "con_system")]
    system: Option<String>,

    #[sqlx(rename = "con_code")]
    code: Option<String>,
}

#[derive(Debug, FromRow)]
struct ObservationRow {
    #[sqlx(rename = "ob_name")]
    name: String,

    #[sqlx(rename = "ob_date")]
    date: NaiveDateTime,

    #[sqlx(rename = "ob_value")]
    value: Option<String>,

    #[sqlx(rename = "ob_system")]
    system: Option<String>,

    #[sqlx(rename = "ob_code")]
    code: Option<String>,

    #[sqlx(rename = "valueCode")]
    value_code: Option<String>,

    #[sqlx(rename = "bodySite")]
    body_site: Option<String>,

    #[sqlx(rename = "ob_status")]
    status: Option<String>,
}

#[derive(Debug, FromRow)]
struct ImmunizationRow {
    #[sqlx(rename = "imm_name")]
    name: String,

    #[sqlx(rename = "imm_system")]
    system: Option<String>,

    #[sqlx(rename = "imm_date")]
    date: NaiveDateTime,

    #[sqlx(rename = "imm_code")]
    code: Option<String>,

    #[sqlx(rename = "imm_status")]
    status: Option<String>,
}

// =================== API JSON MODELS ===================

#[derive(Debug, Serialize, Deserialize)]
pub struct IPSModel {
    pub package_uuid: String,
    pub time_stamp: NaiveDateTime,
    pub patient: Patient,
    pub medications: Vec<Medication>,
    pub allergies: Vec<Allergy>,
    pub conditions: Vec<Condition>,
    pub observations: Vec<Observation>,
    pub immunizations: Vec<Immunization>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Patient {
    pub name: String,
    pub given: String,
    pub dob: NaiveDateTime,
    pub gender: Option<String>,
    pub nation: String,
    pub practitioner: String,
    pub organization: Option<String>,
    pub identifier: Option<String>,
    pub identifier2: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Medication {
    pub name: String,
    pub date: NaiveDateTime,
    pub dosage: String,
    pub system: String,
    pub code: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Allergy {
    pub name: String,
    pub criticality: String,
    pub date: NaiveDateTime,
    pub system: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub date: NaiveDateTime,
    pub system: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Observation {
    pub name: String,
    pub date: NaiveDateTime,
    pub value: String,
    pub system: String,
    pub code: String,
    pub value_code: String,
    pub body_site: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Immunization {
    pub name: String,
    pub system: String,
    pub date: NaiveDateTime,
    pub code: String,
    pub status: String,
}

// =================== SHARED DB LOGIC ===================

async fn load_ips(
    package_uuid: &str,
    pool: &MySqlPool,
) -> Result<Option<IPSModel>, sqlx::Error> {
    // 1. Fetch main IPS row
    let ips_row: Option<IPSRow> =
        sqlx::query_as::<_, IPSRow>(r#"SELECT * FROM `ipsAlt` WHERE `packageUUID` = ? LIMIT 1"#)
            .bind(package_uuid)
            .fetch_optional(pool)
            .await?;

    let Some(ips_row) = ips_row else {
        return Ok(None);
    };

    // 2. Fetch child records using the FK (assumed IPSModelId)
    let medications_rows: Vec<MedicationRow> = sqlx::query_as::<_, MedicationRow>(
        r#"SELECT
               `name`   AS med_name,
               `date`   AS med_date,
               `dosage` AS med_dosage,
               `system` AS med_system,
               `code`   AS med_code,
               `status` AS med_status
           FROM `Medications`
           WHERE `IPSModelId` = ?"#,
    )
    .bind(ips_row.id)
    .fetch_all(pool)
    .await?;

    let allergies_rows: Vec<AllergyRow> = sqlx::query_as::<_, AllergyRow>(
        r#"SELECT
               `name`        AS all_name,
               `criticality` AS all_criticality,
               `date`        AS all_date,
               `system`      AS all_system,
               `code`        AS all_code
           FROM `Allergies`
           WHERE `IPSModelId` = ?"#,
    )
    .bind(ips_row.id)
    .fetch_all(pool)
    .await?;

    let conditions_rows: Vec<ConditionRow> = sqlx::query_as::<_, ConditionRow>(
        r#"SELECT
               `name`   AS con_name,
               `date`   AS con_date,
               `system` AS con_system,
               `code`   AS con_code
           FROM `Conditions`
           WHERE `IPSModelId` = ?"#,
    )
    .bind(ips_row.id)
    .fetch_all(pool)
    .await?;

    let observations_rows: Vec<ObservationRow> = sqlx::query_as::<_, ObservationRow>(
        r#"SELECT
               `name`      AS ob_name,
               `date`      AS ob_date,
               `value`     AS ob_value,
               `system`    AS ob_system,
               `code`      AS ob_code,
               `valueCode` AS valueCode,
               `bodySite`  AS bodySite,
               `status`    AS ob_status
           FROM `Observations`
           WHERE `IPSModelId` = ?"#,
    )
    .bind(ips_row.id)
    .fetch_all(pool)
    .await?;

    let immunizations_rows: Vec<ImmunizationRow> = sqlx::query_as::<_, ImmunizationRow>(
        r#"SELECT
               `name`   AS imm_name,
               `system` AS imm_system,
               `date`   AS imm_date,
               `code`   AS imm_code,
               `status` AS imm_status
           FROM `Immunizations`
           WHERE `IPSModelId` = ?"#,
    )
    .bind(ips_row.id)
    .fetch_all(pool)
    .await?;

    // 3. Map rows into API structs
    let patient = Patient {
        name: ips_row.patient_name,
        given: ips_row.patient_given,
        dob: ips_row.patient_dob,
        gender: ips_row.patient_gender,
        nation: ips_row.patient_nation,
        practitioner: ips_row.patient_practitioner,
        organization: ips_row.patient_organization,
        identifier: ips_row.patient_identifier,
        identifier2: ips_row.patient_identifier2,
    };

    let medications = medications_rows
        .into_iter()
        .map(|m| Medication {
            name: m.name,
            date: m.date,
            dosage: m.dosage,
            system: m.system.unwrap_or_default(),
            code: m.code.unwrap_or_default(),
            status: m.status.unwrap_or_default(),
        })
        .collect();

    let allergies = allergies_rows
        .into_iter()
        .map(|a| Allergy {
            name: a.name,
            criticality: a.criticality.unwrap_or_default(),
            date: a.date,
            system: a.system.unwrap_or_default(),
            code: a.code.unwrap_or_default(),
        })
        .collect();

    let conditions = conditions_rows
        .into_iter()
        .map(|c| Condition {
            name: c.name,
            date: c.date,
            system: c.system.unwrap_or_default(),
            code: c.code.unwrap_or_default(),
        })
        .collect();

    let observations = observations_rows
        .into_iter()
        .map(|o| Observation {
            name: o.name,
            date: o.date,
            value: o.value.unwrap_or_default(),
            system: o.system.unwrap_or_default(),
            code: o.code.unwrap_or_default(),
            value_code: o.value_code.unwrap_or_default(),
            body_site: o.body_site.unwrap_or_default(),
            status: o.status.unwrap_or_default(),
        })
        .collect();

    let immunizations = immunizations_rows
        .into_iter()
        .map(|i| Immunization {
            name: i.name,
            system: i.system.unwrap_or_default(),
            date: i.date,
            code: i.code.unwrap_or_default(),
            status: i.status.unwrap_or_default(),
        })
        .collect();

    let ips_model = IPSModel {
        package_uuid: ips_row.package_uuid,
        time_stamp: ips_row.time_stamp,
        patient,
        medications,
        allergies,
        conditions,
        observations,
        immunizations,
    };

    Ok(Some(ips_model))
}

// =================== ROUTES ===================

// Landing page: simple UUID form
#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

// HTML view that renders IPS nicely
#[get("/ipsview?<uuid>")]
async fn ips_view(uuid: String, pool: &State<MySqlPool>) -> Option<Template> {
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

// Existing JSON API (unchanged behaviour)
#[get("/ips/<package_uuid>")]
async fn get_ips(package_uuid: String, pool: &State<MySqlPool>) -> Option<Json<IPSModel>> {
    let ips = load_ips(&package_uuid, pool.inner())
        .await
        .expect("Failed to load IPS");

    ips.map(Json)
}

// You normally want DELETE, but keeping GET for compatibility
#[get("/ips/delbypra/<practitioner>")]
async fn delete_ips_by_practitioner(practitioner: String, pool: &State<MySqlPool>) -> Json<u64> {
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

// =================== ROCKET SETUP ===================

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv().ok();

    let db_host = env::var("DB_HOST").expect("DB_HOST environment variable not set");
    let db_name = env::var("DB_NAME").expect("DB_NAME environment variable not set");
    let db_user = env::var("DB_USER").expect("DB_USER environment variable not set");
    let db_pass = env::var("DB_PASSWORD").expect("DB_PASSWORD environment variable not set");

    // e.g. mysql://user:pass@localhost/ipsdb
    let database_url = format!("mysql://{}:{}@{}/{}", db_user, db_pass, db_host, db_name);

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create MySQL pool");

    rocket::build()
        .manage(pool)
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                index,
                ips_view,
                get_ips,
                delete_ips_by_practitioner
            ],
        )
}
