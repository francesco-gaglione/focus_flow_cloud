use crate::entities::tasks::category::Category;

pub struct CountByCategory {
    counts: Vec<CategoryCount>,
}

pub struct CategoryCount {
    category: Category,
    count: u64,
}

impl CategoryCount {
    pub fn new(category: Category, count: u64) -> Self {
        Self { category, count }
    }

    pub fn category(&self) -> &Category {
        &self.category
    }

    pub fn count(&self) -> u64 {
        self.count
    }
}

impl CountByCategory {
    pub fn new(counts: Vec<CategoryCount>) -> Self {
        Self { counts }
    }

    pub fn counts(&self) -> &[CategoryCount] {
        &self.counts
    }

    pub fn total(&self) -> u64 {
        self.counts.iter().map(|c| c.count).sum()
    }

    pub fn find_by_category_id(&self, id: uuid::Uuid) -> Option<&CategoryCount> {
        self.counts.iter().find(|c| c.category.id() == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn make_category(name: &str) -> Category {
        Category::create(Uuid::new_v4(), name.to_string(), "#FF0000".to_string()).unwrap()
    }

    #[test]
    fn test_category_count_getters() {
        let cat = make_category("Work");
        let id = cat.id();
        let cc = CategoryCount::new(cat, 5);
        assert_eq!(cc.count(), 5);
        assert_eq!(cc.category().id(), id);
    }

    #[test]
    fn test_count_by_category_total() {
        let counts = vec![
            CategoryCount::new(make_category("Work"), 3),
            CategoryCount::new(make_category("Personal"), 7),
        ];
        let cbc = CountByCategory::new(counts);
        assert_eq!(cbc.total(), 10);
    }

    #[test]
    fn test_count_by_category_empty_total() {
        let cbc = CountByCategory::new(vec![]);
        assert_eq!(cbc.total(), 0);
    }

    #[test]
    fn test_find_by_category_id_found() {
        let cat = make_category("Work");
        let id = cat.id();
        let cbc = CountByCategory::new(vec![CategoryCount::new(cat, 2)]);
        assert!(cbc.find_by_category_id(id).is_some());
        assert_eq!(cbc.find_by_category_id(id).unwrap().count(), 2);
    }

    #[test]
    fn test_find_by_category_id_not_found() {
        let cbc = CountByCategory::new(vec![CategoryCount::new(make_category("Work"), 1)]);
        assert!(cbc.find_by_category_id(Uuid::new_v4()).is_none());
    }

    #[test]
    fn test_counts_slice() {
        let counts = vec![
            CategoryCount::new(make_category("A"), 1),
            CategoryCount::new(make_category("B"), 2),
        ];
        let cbc = CountByCategory::new(counts);
        assert_eq!(cbc.counts().len(), 2);
    }
}
