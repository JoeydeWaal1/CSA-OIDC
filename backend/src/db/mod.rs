use dashmap::DashMap;
use std::sync::Arc;
use serde::Serialize;

#[derive(Debug,Serialize,Default)]
pub struct Row {
    pub todos: Vec<String>
}

#[derive(Clone)]
pub struct InMemDatabase(
    pub Arc<DashMap<String, Row>>
);

impl InMemDatabase {
    pub fn new() -> Self {
        Self(Arc::new(DashMap::new()))
    }
}