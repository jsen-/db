use rusqlite as sqlite;

use super::Queryable;

impl<'a, I> Queryable<sqlite::Connection> for I
where
    I: IntoIterator,
    I::Item: sqlite::types::ToSql,
{
    type Error = sqlite::Error;

    fn query(self, sql: &str, conn: &sqlite::Connection) -> Result<(), Self::Error> {
        let mut stmt = conn.prepare(sql).unwrap();
        stmt.query(self.into_iter()).unwrap();
        println!("sqlite");
        Ok(())
    }
}
