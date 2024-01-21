use serde::{Deserialize, Serialize};

use crate::{subscription::Subscription, visit::Visit};

#[derive(Debug, Serialize, Deserialize)]
pub enum Entry {
    Visit(Visit),
    Subscription(Subscription),
}
