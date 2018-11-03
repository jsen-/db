mod oracle;
mod sqlite;

pub trait Queryable<C> {
    type Error;

    fn query(self, sql: &str, conn: &C) -> Result<(), Self::Error>;
}

pub trait Fetch<C> {
    fn fetch<Q, S>(&self, sql: S, p: Q) -> Result<(), Q::Error>
    where
        Q: Queryable<C>,
        S: AsRef<str>;
}

impl<C> Fetch<C> for C {
    fn fetch<Q, S>(&self, sql: S, p: Q) -> Result<(), Q::Error>
    where
        Q: Queryable<C>,
        S: AsRef<str>,
    {
        p.query(sql.as_ref(), self)
    }
}