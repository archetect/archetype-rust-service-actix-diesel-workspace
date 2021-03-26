#[derive(Queryable)]
pub struct {{ PrefixName }} {
    pub id: i32,
    pub name: String,
    pub created: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
