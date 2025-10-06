use crate::adapters::http::dto::category_api::create_category::CreateCategoryDto;
use crate::domain::entities::category::Category;
use uuid::Uuid;

pub struct CategoryMapper;

impl CategoryMapper {
    pub fn from_create_dto(dto: CreateCategoryDto) -> Category {
        Category {
            id: Uuid::new_v4(),
            name: dto.name,
            description: dto.description,
            color: dto.color,
        }
    }

    pub fn from_update_dto(dto: CreateCategoryDto, existing_id: Uuid) -> Category {
        Category {
            id: existing_id,
            name: dto.name,
            description: dto.description,
            color: dto.color,
        }
    }
}
