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

use soroban_sdk::{Env, String};

use crate::Lesson;

#[test]
fn test_create_lesson() {
    let env = Env::default();

    let lesson = Lesson {
        lesson_id: 1,
        course_id: 100,
        title: String::from_str(&env, "Introduction"),
        description: String::from_str(&env, "Welcome to the course"),
        content_uri: String::from_str(&env, "ipfs://QmLessonHash"),
        estimated_duration: 30,
        lesson_order: 1,
    };

    assert_eq!(lesson.lesson_id, 1);
    assert_eq!(lesson.course_id, 100);
    assert_eq!(lesson.title, String::from_str(&env, "Introduction"));
    assert_eq!(lesson.estimated_duration, 30);
    assert_eq!(lesson.lesson_order, 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_quiz() {
        let quiz = Quiz {
            quiz_id: 1,
            lesson_id: 10,
            passing_score: 70,
            maximum_score: 100,
            reward_points: 50,
            is_active: true,
        };

        assert_eq!(quiz.quiz_id, 1);
        assert_eq!(quiz.lesson_id, 10);
        assert_eq!(quiz.passing_score, 70);
        assert_eq!(quiz.maximum_score, 100);
        assert_eq!(quiz.reward_points, 50);
        assert!(quiz.is_active);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_quiz() {
        let quiz = Quiz {
            quiz_id: 1,
            lesson_id: 10,
            passing_score: 70,
            maximum_score: 100,
            reward_points: 50,
            is_active: true,
        };

        assert_eq!(quiz.quiz_id, 1);
        assert_eq!(quiz.lesson_id, 10);
        assert_eq!(quiz.passing_score, 70);
        assert_eq!(quiz.maximum_score, 100);
        assert_eq!(quiz.reward_points, 50);
        assert!(quiz.is_active);
    }
}