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