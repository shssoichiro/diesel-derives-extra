# diesel_derives_extra

Automatically derive some simple CRUD methods for your Diesel models.

## Basic usage

- Add `diesel_derives_extra` to your `Cargo.toml`
- Add `#[macro_use] extern crate diesel_derives_extra;` and `extern crate diesel_derives_traits;` to your project's entry point

### Models

For models that are used for representing data in the database, you can use the following:

```rust
use diesel_derives_traits::Model;

#[derive(Debug, Queryable, Identifiable, AsChangeset, Model)]
pub struct User {
    // fields omitted
}
```

`Model` is the new derive added by this crate. The others are required to make the `Model` trait work.

This generates the following methods:

```rust
fn save(self, conn: &PgConnection) -> QueryResult<Self>;
fn find_all(conn: &PgConnection) -> QueryResult<Vec<Self>>;
fn find_one(
    conn: &PgConnection,
    id: <&'a Self as Identifiable>::Id,
) -> QueryResult<Option<Self>>;
fn exists(conn: &PgConnection, id: <&'a Self as Identifiable>::Id) -> QueryResult<bool>;
fn count_all(conn: &PgConnection) -> QueryResult<i64>;
fn destroy(self, conn: &PgConnection) -> QueryResult<()>;
``` 

### New Models

For models that are used for inserting new data into a table, you can use the following:

```rust
use diesel_derives_traits::NewModel;

#[derive(Debug, Insertable, NewModel)]
#[model(User)]
struct NewUser {
    // fields omitted
}
```

This generates one method:

```rust
fn save(self, conn: &PgConnection) -> QueryResult<T>;
```