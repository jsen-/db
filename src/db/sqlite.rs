use rusqlite as sqlite;

use super::{Preparable, Queryable};

impl<'a, I> Queryable<sqlite::Statement<'a>> for I
where
    I: IntoIterator,
    I::Item: sqlite::types::ToSql,
{
    type Error = sqlite::Error;

    fn query(self, stmt: &mut sqlite::Statement) -> Result<(), Self::Error> {
        stmt.query(self.into_iter())?;
        Ok(())
    }
}

impl<'a> Preparable<'a, sqlite::Statement<'a>> for sqlite::Connection {
    type Error = sqlite::Error;
    fn prep<SQL: AsRef<str>>(&'a self, sql: SQL) -> Result<sqlite::Statement<'a>, Self::Error> {
        self.prepare("")
    }
}
