#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate diesel;

use diesel::prelude::*;
use diesel::associations::HasTable;

pub trait Model<'a>
where
    &'a Self: Identifiable,
    Self: Sized + 'a
{
    fn save(self, conn: &::diesel::PgConnection) -> ::diesel::result::QueryResult<Self>;
    fn find_all(conn: &::diesel::PgConnection) -> ::diesel::result::QueryResult<Vec<Self>>;
    fn find_one(
        conn: &::diesel::PgConnection,
        id: <&'a Self as ::diesel::Identifiable>::Id,
    ) -> ::diesel::result::QueryResult<Option<Self>>;
    fn count_all(conn: &::diesel::PgConnection) -> ::diesel::result::QueryResult<i64>;
    fn destroy(self, conn: &::diesel::PgConnection) -> ::diesel::result::QueryResult<()>;
}

//pub trait NewModel<T>: Insertable<T>
//where
//    T: ::diesel::associations::HasTable,
//{
//    fn save(self, conn: &::diesel::PgConnection) -> ::diesel::result::QueryResult<T>;
//}
