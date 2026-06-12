use async_trait::async_trait;
use domain::shared::entities::{folder::Folder, folder_metadata::FolderMetadata};
use uuid::Uuid;

use crate::shared::traits::persistence_error::PersistenceResult;

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait FolderPersistence: Send + Sync {
    async fn create_folder(&self, parent_id: Uuid) -> PersistenceResult<Uuid>;

    async fn delete_folder(&self, folder_id: Uuid) -> PersistenceResult<()>;

    async fn update_folder(&self, folder_metadata: Folder<FolderMetadata>)
        -> PersistenceResult<()>;

    async fn read_folder_by_id(&self, folder_id: Uuid)
        -> PersistenceResult<Folder<FolderMetadata>>;
}
