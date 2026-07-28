use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Course {
    pub course_id: u64,
    pub title: String,
    pub description: String,
    pub category: String,
    pub difficulty: String,
    pub thumbnail_hash: String,
    pub author: Address,
    pub published: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

use soroban_sdk::{contracttype, Address, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Module {
    pub module_id: u64,
    pub course_id: u64,
    pub title: String,
    pub lesson_ids: Vec<u64>,
    pub display_order: u32,
}