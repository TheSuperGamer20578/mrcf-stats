#![warn(clippy::pedantic)]

use std::env;
use chrono::Utc;
use dotenv::dotenv;
use sqlx::{Connection, MySqlConnection, query};
#[cfg(feature = "modrinth")]
use ferinth::Ferinth;
#[cfg(feature = "curseforge")]
use curseforge::endpoints::DEFAULT_API_BASE;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut conn = MySqlConnection::connect(env::var("DATABASE_URL").unwrap().as_str()).await.unwrap();
    sqlx::migrate!().run(&mut conn).await.unwrap();
    let timestamp = Utc::now();
    #[cfg(feature = "modrinth")]
    if let Ok(mr_token) = env::var("MR_TOKEN") {
        let mr_api = Ferinth::new(
            env!("CARGO_CRATE_NAME"),
            Some(env!("CARGO_PKG_VERSION")),
            Some("discord.gg/ehqQUvvmc6"),
            Some(&mr_token),
        ).unwrap();
        let user = mr_api.get_current_user().await.unwrap();
        let payouts = mr_api.payout_history(&user.id).await.unwrap();
        let projects = mr_api.list_projects(&user.id).await.unwrap();
        let version_ids: Vec<_> = projects.iter()
            .flat_map(|project| &project.versions)
            .map(String::as_str)
            .collect();
        let versions = mr_api.get_multiple_versions(&version_ids).await.unwrap();
        query!(/* language=mariadb */ "INSERT INTO mr_payouts (timestamp, amount) VALUES (?, ?);",
            timestamp, payouts.all_time)
            .execute(&mut conn).await.unwrap();
        for project in projects {
            query!(/* language=mariadb */ "
                INSERT INTO mr_projects (id, name, type)
                VALUES (?, ?, ?)
                ON DUPLICATE KEY UPDATE name=VALUES(name), type=VALUES(type);
                ", project.id, project.title, project.project_type as u8)
                .execute(&mut conn).await.unwrap();
            query!(/* language=mariadb */ "
                INSERT INTO mr_project_downloads (timestamp, project, downloads)
                VALUES (?, ?, ?);
                ", timestamp, project.id, project.downloads.to_string())
                .execute(&mut conn).await.unwrap();
        }
        for version in versions {
            query!(/* language=mariadb */ "
                INSERT INTO mr_versions (id, project, name, number, release_date)
                VALUES (?, ?, ?, ?, ?)
                ON DUPLICATE KEY UPDATE name=VALUES(name), number=VALUES(number);
                ", version.id, version.project_id, version.name, version.version_number, version.date_published)
                .execute(&mut conn).await.unwrap();
            query!(/* language=mariadb */ "
                INSERT INTO mr_version_downloads (timestamp, version, downloads)
                VALUES (?, ?, ?);
                ", timestamp, version.id, version.downloads.to_string())
                .execute(&mut conn).await.unwrap();
        }
    }
    #[cfg(feature = "curseforge")]
    if let Ok(cf_token) = env::var("CF_TOKEN") {
        let cf_api = curseforge::Client::new(DEFAULT_API_BASE, Some(cf_token), None).unwrap();
        compile_error!("CurseForge is not yet implemented, please remove the `curseforge` feature flag.");
    }
}
