use std::{
    fs::{self, File},
    io::prelude::{Read, Write},
};

use crate::{
    subscription::{Subscription, SubscriptionType},
    tui_entry::Entry,
    visit::{Visit, VisitType},
};
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct SubscriptionApp {
    entries: Vec<Entry>,
    file_handler: File,
}

impl SubscriptionApp {
    const FILE_PATH: &str = "sub_file";
    // init
    pub fn new() -> Result<Self> {
        // try read file
        let mut file_handler = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(Self::FILE_PATH)?;
        let mut buf = String::new();
        file_handler.read_to_string(&mut buf)?;
        let mut entries = vec![];
        if !buf.is_empty() {
            entries = serde_json::from_str(&buf)?;
        }

        Ok(Self {
            entries,
            file_handler,
        })
    }
    // buy new subscription
    pub fn buy_subscription(&mut self, sub_type: SubscriptionType) {
        self.entries
            .push(Entry::Subscription(Subscription::new(sub_type)));
        // writeln!(self.file_handler, "");
    }
    // add visit
    pub fn visited(&mut self, visit_type: VisitType) {
        self.entries.push(Entry::Visit(Visit::new(visit_type)))
    }
    // remove entry
    pub fn remove_entry(entry_id: u64) -> Result<()> {
        todo!()
    }
    pub fn save_content(&mut self) -> Result<()> {
        self.file_handler
            .write_all(serde_json::to_string(&self.entries)?.as_bytes())?;
        Ok(())
    }
}
