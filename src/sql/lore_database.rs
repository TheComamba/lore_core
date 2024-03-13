use std::path::PathBuf;

use crate::errors::LoreCoreError;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub struct LoreDatabase {
    path: PathBuf,
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

impl LoreDatabase {
    pub fn open(path: PathBuf) -> Result<Self, LoreCoreError> {
        let db = LoreDatabase { path };
        db.db_connection()?
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| {
                LoreCoreError::SqlError(
                    "Failed to run SQL database migrations: ".to_string() + &e.to_string(),
                )
            })?;
        Ok(db)
    }

    pub fn path_as_string(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    pub(super) fn db_connection(&self) -> Result<SqliteConnection, LoreCoreError> {
        let path = match self.path.to_str() {
            Some(str) => str,
            None => return Err(LoreCoreError::FileError(
                "Could not open database path.".to_string()
                    + "This is likely because it contains characters that can not be UTF-8 encoded."
                    + "The lossy path conversion reads:\n"
                    + &self.path.to_string_lossy(),
            )),
        };
        SqliteConnection::establish(path).map_err(|e| {
            LoreCoreError::SqlError(
                "Failed to establish a connection to the database: ".to_string() + &e.to_string(),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;
    use std::path::PathBuf;

    #[test]
    fn test_open_non_existing_database() {
        let non_existing_path = PathBuf::from("/non/existing/path");
        let result = LoreDatabase::open(non_existing_path);
        assert!(result.is_err());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_open_database_with_non_utf8_path() {
        let non_utf8_path =
            unsafe { PathBuf::from(OsString::from_encoded_bytes_unchecked(vec![0xFF, 0xFF])) };
        let result = LoreDatabase::open(non_utf8_path);
        assert!(matches!(result, Err(LoreCoreError::FileError(_))));
    }
}
