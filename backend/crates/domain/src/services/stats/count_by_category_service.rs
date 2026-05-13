use crate::{
    entities::tasks::{category::Category, task::Task},
    value_objects::stats::count_by_category::{CategoryCount, CountByCategory},
};

pub struct CountByCategoryService {}

impl CountByCategoryService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate(tasks: &[Task], categories: &[Category]) -> CountByCategory {
        let counts = categories
            .iter()
            .map(|cat| {
                let count = tasks
                    .iter()
                    .filter(|t| {
                        t.completed_at().is_some() && t.category_id() == Some(cat.id())
                    })
                    .count() as u64;
                CategoryCount::new(cat.clone(), count)
            })
            .collect();

        CountByCategory::new(counts)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::entities::tasks::{category::Category, task::Task, task_schedule::TaskSchedule};

    use super::*;

    fn category(name: &str) -> Category {
        Category::create(Uuid::new_v4(), name.to_string(), "#FF0000".to_string()).unwrap()
    }

    fn completed_task_with_category(category_id: Uuid) -> Task {
        let mut t = Task::new(Uuid::new_v4(), "task".to_string(), TaskSchedule::Unscheduled, None);
        t.update_category_id(category_id);
        t.complete().unwrap();
        t
    }

    fn pending_task_with_category(category_id: Uuid) -> Task {
        let mut t = Task::new(Uuid::new_v4(), "task".to_string(), TaskSchedule::Unscheduled, None);
        t.update_category_id(category_id);
        t
    }

    #[test]
    fn test_empty() {
        let result = CountByCategoryService::calculate(&[], &[]);
        assert_eq!(result.total(), 0);
    }

    #[test]
    fn test_counts_per_category() {
        let cat_a = category("Work");
        let cat_b = category("Personal");
        let tasks = vec![
            completed_task_with_category(cat_a.id()),
            completed_task_with_category(cat_a.id()),
            completed_task_with_category(cat_b.id()),
        ];
        let result = CountByCategoryService::calculate(&tasks, &[cat_a.clone(), cat_b.clone()]);
        assert_eq!(result.find_by_category_id(cat_a.id()).unwrap().count(), 2);
        assert_eq!(result.find_by_category_id(cat_b.id()).unwrap().count(), 1);
        assert_eq!(result.total(), 3);
    }

    #[test]
    fn test_pending_tasks_not_counted() {
        let cat = category("Work");
        let tasks = vec![
            completed_task_with_category(cat.id()),
            pending_task_with_category(cat.id()),
        ];
        let result = CountByCategoryService::calculate(&tasks, &[cat.clone()]);
        assert_eq!(result.find_by_category_id(cat.id()).unwrap().count(), 1);
    }

    #[test]
    fn test_category_not_in_list_not_counted() {
        let cat = category("Work");
        let other_cat_id = Uuid::new_v4();
        let tasks = vec![completed_task_with_category(other_cat_id)];
        let result = CountByCategoryService::calculate(&tasks, &[cat.clone()]);
        assert_eq!(result.find_by_category_id(cat.id()).unwrap().count(), 0);
        assert_eq!(result.total(), 0);
    }
}
