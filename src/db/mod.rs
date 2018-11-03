mod oracle;
mod sqlite;

pub trait Queryable<STMT> {
    type Error;

    fn query(self, stmt: &mut STMT) -> Result<(), Self::Error>;
}

pub trait Preparable<'a, S> {
    type Error;
    fn prep<SQL: AsRef<str>>(&'a self, sql: SQL) -> Result<S, Self::Error>;
}

pub trait Fetch<STMT> {
    fn fetch<Q>(&mut self, p: Q) -> Result<(), Q::Error>
    where
        Q: Queryable<STMT>;
}

impl<STMT> Fetch<STMT> for STMT {
    fn fetch<Q>(&mut self, p: Q) -> Result<(), Q::Error>
    where
        Q: Queryable<STMT>,
    {
        p.query(self)
    }
}
