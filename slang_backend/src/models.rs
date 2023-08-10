use diesel::prelude::*;

/*
TODO BOARD

json handling of objects
*/

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::slang_groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    group_id: i32,
    group_name: String,
    group_public: bool,
    group_admins: Vec<Option<i32>>,
    group_banlist: Vec<Option<i32>>,
    group_roles: Vec<Option<i32>>
}
