#![allow(unused_imports, dead_code, unused_variables)]

use oracle;
use rusqlite as sqlite;

mod db;
pub use self::db::{Fetch, Preparable};

#[derive(Debug)]
enum DbError {
    Oracle(oracle::Error),
    Sqlite(sqlite::Error),
}

impl From<oracle::Error> for DbError {
    fn from(oraerr: oracle::Error) -> Self {
        DbError::Oracle(oraerr)
    }
}
impl From<sqlite::Error> for DbError {
    fn from(sqliteerr: sqlite::Error) -> Self {
        DbError::Sqlite(sqliteerr)
    }
}

fn main() -> Result<(), DbError> {
    let ora = oracle::Connection::connect("system", "vagrant", "localhost:1521/xe", &[]).unwrap();
    let sql = sqlite::Connection::open_in_memory().unwrap();

    ora.prepare("", &[])?.fetch(&[0, 1])?;
    sql.prepare("")?.fetch(&[0, 1])?;

    Ok(())
}
