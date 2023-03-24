use super::schema::heroes;

#[derive(serde::Serialize, serde::Deserialize, Queryable)]
pub struct Hero {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
    #[serde(skip_deserializing)]
    pub updated_at: String,
}

#[derive(serde::Deserialize, Insertable)]
#[table_name = "heroes"]
pub struct NewHero {
    pub name: String,
    pub email: String,
}
