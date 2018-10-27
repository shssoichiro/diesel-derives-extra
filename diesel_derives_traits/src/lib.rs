#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate diesel;

use diesel::associations::HasTable;
use diesel::prelude::*;

pub trait Model<'a, C>
where
    &'a Self: Identifiable,
    C: Connection,
    Self: Sized + 'a,
{
    fn save(self, conn: &C) -> QueryResult<Self>;
    fn find_all(conn: &C) -> QueryResult<Vec<Self>>;
    fn find_one(conn: &C, id: <&'a Self as Identifiable>::Id) -> QueryResult<Option<Self>>;
    fn exists(conn: &C, id: <&'a Self as Identifiable>::Id) -> QueryResult<bool>;
    fn count_all(conn: &C) -> QueryResult<i64>;
    fn destroy(self, conn: &C) -> QueryResult<()>;
}

pub trait NewModel<'a, T, C>
where
    &'a T: HasTable,
    T: 'a,
    C: Connection,
    &'a Self: Insertable<<&'a T as HasTable>::Table>,
    Self: 'a,
{
    fn save(self, conn: &C) -> QueryResult<T>;
}
