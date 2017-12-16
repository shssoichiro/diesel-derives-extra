#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate diesel;

use diesel::prelude::*;
use diesel::associations::HasTable;

pub trait Model<'a>
where
    &'a Self: Identifiable,
    Self: Sized + 'a,
{
    fn save(self, conn: &PgConnection) -> QueryResult<Self>;
    fn find_all(conn: &PgConnection) -> QueryResult<Vec<Self>>;
    fn find_one(
        conn: &PgConnection,
        id: <&'a Self as Identifiable>::Id,
    ) -> QueryResult<Option<Self>>;
    fn exists(conn: &PgConnection, id: <&'a Self as Identifiable>::Id) -> QueryResult<bool>;
    fn count_all(conn: &PgConnection) -> QueryResult<i64>;
    fn destroy(self, conn: &PgConnection) -> QueryResult<()>;
}

//pub trait NewModel<T>: Insertable<T>
//where
//    T: ::diesel::associations::HasTable,
//{
//    fn save(self, conn: &PgConnection) -> QueryResult<T>;
//}
