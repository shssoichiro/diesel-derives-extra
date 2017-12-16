#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate diesel;

pub trait Model<'a> where &'a Self: ::diesel::Identifiable, Self: 'a + ::diesel::associations::HasTable + Sized {
    fn save(self, conn: &::diesel::PgConnection) -> ::diesel::result::QueryResult<Self>;
    fn find_all(conn: &::diesel::PgConnection) -> ::diesel::result::QueryResult<Vec<Self>>;
    fn find_one(conn: &::diesel::PgConnection, id: <&'a Self as ::diesel::associations::Identifiable>::Id) -> ::diesel::result::QueryResult<Option<Self>>;
    fn exists(conn: &::diesel::PgConnection, id: <&'a Self as ::diesel::associations::Identifiable>::Id) -> ::diesel::result::QueryResult<bool>;
    fn count_all(conn: &::diesel::PgConnection) -> ::diesel::result::QueryResult<i64>;
    fn destroy(self, conn: &::diesel::PgConnection) -> ::diesel::result::QueryResult<()>;
}

pub trait NewModel<T: ::diesel::associations::HasTable> where Self: ::diesel::prelude::Insertable<T> {
    fn save(self, conn: &::diesel::PgConnection) -> ::diesel::result::QueryResult<T>;
}