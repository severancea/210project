// ds210 final project
// andrew severance | asev@bu.edu

// models.rs builds upon data.rs and allows the columns within the data set to be analyzed.
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct RaceData {
    race_id: usize,
    driver_id: usize,
    driver_ref: String,
}