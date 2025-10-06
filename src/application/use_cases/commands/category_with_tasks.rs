use crate::domain::entities::category::Category;
use crate::domain::entities::task::Task;

pub struct CategoryWithTasks {
    pub category: Category,
    pub tasks: Vec<Task>,
}

pub struct CategoryAndTasks {
    pub category_with_tasks: Vec<CategoryWithTasks>,
    pub orphan_tasks: Vec<Task>,
}
