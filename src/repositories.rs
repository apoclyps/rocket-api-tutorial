use crate::{models::Hero, models::NewHero, schema::heroes};
use diesel::prelude::*;
use diesel::SqliteConnection;

pub struct HeroesRepository;

impl HeroesRepository {
    pub fn find(c: &SqliteConnection, id: i32) -> QueryResult<Hero> {
        heroes::table.find(id).get_result::<Hero>(c)
    }

    pub fn all(c: &SqliteConnection, limit: i64) -> QueryResult<Vec<Hero>> {
        heroes::table.limit(limit).load::<Hero>(c)
    }

    pub fn create(c: &SqliteConnection, new_hero: NewHero) -> QueryResult<Hero> {
        diesel::insert_into(heroes::table)
            .values(new_hero)
            .execute(c)?;

        let last_id = Self::last_inserted_id(c)?;
        Self::find(c, last_id)
    }

    pub fn save(c: &SqliteConnection, id: i32, hero: Hero) -> QueryResult<Hero> {
        diesel::update(heroes::table.find(id))
            .set((heroes::name.eq(hero.name), heroes::email.eq(hero.email)))
            .execute(c)?;

        Self::find(c, id)
    }

    pub fn delete(c: &SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(heroes::table.find(id)).execute(c)
    }

    fn last_inserted_id(c: &SqliteConnection) -> QueryResult<i32> {
        heroes::table
            .select(heroes::id)
            .order(heroes::id.desc())
            .first(c)
    }
}
