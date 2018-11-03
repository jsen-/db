use super::Queryable;
use oracle;

pub trait MyToSql: oracle::ToSql {
    fn as_ora(&self) -> &oracle::ToSql;
}
impl<T: oracle::ToSql> MyToSql for T {
    fn as_ora(&self) -> &oracle::ToSql {
        self
    }
}

impl<'a, T: MyToSql> From<&'a T> for &'a dyn MyToSql {
    fn from(t: &'a T) -> Self {
        t as &MyToSql
    }
}

impl<'a, I> Queryable<oracle::Connection> for I
where
    I: IntoIterator,
    I::Item: Into<&'a MyToSql>,
{
    type Error = oracle::Error;
    fn query(self, sql: &str, conn: &oracle::Connection) -> Result<(), Self::Error> {
        let mut stmt = conn.prepare(sql, &[]).unwrap();
        let params: Vec<&dyn oracle::ToSql> = self.into_iter().map(Into::into).map(MyToSql::as_ora).collect();
        stmt.query(params.as_slice()).unwrap();
        println!("oracle");
        Ok(())
    }
}
