mod item;
use crate::{db::DataManager, models::Region};

#[derive(Debug)]
pub struct Market<DB: DataManager> {
    region: Region,
    db: DB,
}


