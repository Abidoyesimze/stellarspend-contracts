#![cfg(test)]

use soroban_sdk::{Env, String, Vec};

use crate::{LMSContract, Module};

#[test]
fn test_initialize() {
    let _env = Env::default();

    let result = LMSContract::initialize();

    assert!(result);
}

#[test]
fn test_create_module() {
    let env = Env::default();

    let mut lessons = Vec::new(&env);

    lessons.push_back(1);
    lessons.push_back(2);
    lessons.push_back(3);

    let module = Module {
        module_id: 1,
        course_id: 100,
        title: String::from_str(&env, "Introduction"),
        lesson_ids: lessons.clone(),
        display_order: 1,
    };

    assert_eq!(module.module_id, 1);
    assert_eq!(module.course_id, 100);
    assert_eq!(module.lesson_ids.len(), 3);
    assert_eq!(module.display_order, 1);

    assert_eq!(module.lesson_ids.get(0), Some(1));
    assert_eq!(module.lesson_ids.get(1), Some(2));
    assert_eq!(module.lesson_ids.get(2), Some(3));
}