#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derives_extra;
extern crate diesel_derives_traits;
#[cfg(feature="logger")]
extern crate diesel_logger;

#[test]
fn simple_model() {
    table! {
        jobs (id) {
            id -> Int4,
            payload -> Varchar,
        }
    }

    #[derive(Debug, Queryable, Identifiable, AsChangeset, Model)]
    #[table_name = "jobs"]
    struct Job {
        id: i32,
        payload: String,
    }

    #[derive(Debug, Insertable, NewModel)]
    #[table_name = "jobs"]
    #[model(Job)]
    struct NewJob {
        payload: String,
    }
}

#[test]
fn with_lifetime() {
    table! {
        jobs (id) {
            id -> Int4,
            payload -> Varchar,
        }
    }

    #[derive(Debug, Queryable, Identifiable, AsChangeset, Model)]
    #[table_name = "jobs"]
    struct Job {
        id: i32,
        payload: String,
    }

    #[derive(Debug, Insertable, NewModel)]
    #[table_name = "jobs"]
    #[model(Job)]
    struct NewJob<'a> {
        payload: &'a str,
    }
}

#[test]
fn new_without_model() {
    table! {
        jobs (id) {
            id -> Int4,
            payload -> Varchar,
        }
    }

    #[derive(Debug, Queryable, Identifiable)]
    #[table_name = "jobs"]
    struct Job {
        id: i32,
        payload: String,
    }

    #[derive(Debug, Insertable, NewModel)]
    #[table_name = "jobs"]
    #[model(Job)]
    struct NewJob {
        payload: String,
    }
}
