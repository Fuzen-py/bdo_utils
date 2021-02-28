mod item;
use crate::{db::DataManager, marketplace::Region};

#[derive(Debug)]
pub struct Market<DB: DataManager> {
    region: Region,
    db: DB,
}
