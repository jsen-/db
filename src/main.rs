use oracle;
use rusqlite as sqlite;

mod db;
pub use self::db::Fetch;

fn main() {
    let ora = oracle::Connection::connect("system", "vagrant", "localhost:1521/xe", &[]).unwrap();
    let sql = sqlite::Connection::open_in_memory().unwrap();

    ora.fetch("", &[0, 1]).unwrap();
    sql.fetch("", &[&0 as &sqlite::types::ToSql, &""]).unwrap();
}
