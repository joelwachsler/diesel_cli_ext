#[derive(Queryable, Debug)]
#[diesel(table_name = series)]
pub struct Series {
    pub id: i32,
}
