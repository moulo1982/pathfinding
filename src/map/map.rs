use crate::errors::my_errors::RetResult;
use crate::map::{Point, PointType};

pub type MapType = std::sync::Arc<std::sync::RwLock<dyn Map>>;

pub trait Map : Send + Sync {

    fn new() -> MapType where Self: Sized;

    fn load(&mut self, points: Vec<Vec<i32>>) -> RetResult<()>;

    //async fn load_from_file(&mut self, points: String) -> RetResult<()>;

    fn find_path(&self, start: PointType, end: PointType) -> Vec<Point>;

    fn in_map(&self, point: &PointType) -> bool;
}