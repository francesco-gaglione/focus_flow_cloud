import type { FlashcardDto } from './FlashcardDto';
import type { FolderDto } from './FolderDto';

export interface RootFolderContentsResponseDto {
  folderId: string;
  folders: FolderDto[];
  flashcards: FlashcardDto[];
}
