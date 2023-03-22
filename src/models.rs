#[derive(serde::Serialize, Queryable)]
pub struct Hero {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}
