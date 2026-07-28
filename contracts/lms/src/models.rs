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