use crate::state::DirectoryInfo;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::{
    SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions,
};
use sqlx::{Pool, Sqlite};
use std::str::FromStr;
use std::time::Duration;

pub(crate) async fn connect() -> crate::Result<Pool<Sqlite>> {
    let settings_dir = DirectoryInfo::get_initial_settings_dir().ok_or(
        crate::ErrorKind::FSError(
            "Could not find valid config dir".to_string(),
        ),
    )?;

    if !settings_dir.exists() {
        crate::util::io::create_dir_all(&settings_dir).await?;
    }

    let uri = format!("sqlite:{}", settings_dir.join("app.db").display());

    if !Sqlite::database_exists(&uri).await? {
        Sqlite::create_database(&uri).await?;
    }

    let conn_options = SqliteConnectOptions::from_str(&uri)?
        .busy_timeout(Duration::from_secs(30))
        .journal_mode(SqliteJournalMode::Wal)
        .optimize_on_close(true, None);

    let pool = SqlitePoolOptions::new()
        .max_connections(100)
        .connect_with(conn_options)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    if let Some((old_dir, new_dir)) = DirectoryInfo::take_app_dir_migration() {
        update_moved_paths(&pool, &old_dir, &new_dir).await?;
    }

    Ok(pool)
}

/// Rewrites absolute paths stored in the database after the data folder was renamed
async fn update_moved_paths(
    pool: &Pool<Sqlite>,
    old_dir: &std::path::Path,
    new_dir: &std::path::Path,
) -> crate::Result<()> {
    let old_prefix = old_dir.to_string_lossy().to_string();
    let new_prefix = new_dir.to_string_lossy().to_string();

    for (table, column) in [
        ("java_versions", "path"),
        ("profiles", "icon_path"),
        ("profiles", "override_java_path"),
        ("settings", "custom_dir"),
        ("settings", "prev_custom_dir"),
    ] {
        let query = format!(
            "UPDATE {table} SET {column} = ?2 || substr({column}, length(?1) + 1)              WHERE substr({column}, 1, length(?1)) = ?1"
        );
        sqlx::query(&query)
            .bind(&old_prefix)
            .bind(&new_prefix)
            .execute(pool)
            .await?;
    }

    Ok(())
}
