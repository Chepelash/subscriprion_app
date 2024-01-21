use chrono::{NaiveDate, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Visit {
    date: NaiveDate,
    visit_type: VisitType,
}

impl Visit {
    pub fn new(visit_type: VisitType) -> Self {
        Self {
            date: Utc::now().date_naive(),
            visit_type,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VisitType {
    Functional,
    CrossFit,
}
