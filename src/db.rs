use diesel::SqliteConnection;

#[database("sqlite_users")]
pub struct Users(SqliteConnection);

#[derive(Queryable)]
pub struct User {
    pub login: String,
    pub password: String
}
