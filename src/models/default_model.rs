use diesel::sql_types::Date;

pub trait DefaultModelTrait {
    fn get_id(&self) -> i32;
    fn get_created_at(&self) -> Date;
    fn get_created_by(&self) -> i32;
    fn get_updated_at(&self) -> Date;
    fn get_updated_by(&self) -> i32;
    fn get_deleted_at(&self) -> Date;
    fn get_deleted_by(&self) -> i32;
}
