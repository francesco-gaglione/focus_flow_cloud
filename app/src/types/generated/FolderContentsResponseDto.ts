import type { FlashcardDto } from './FlashcardDto';
import type { FolderDto } from './FolderDto';

export interface FolderContentsResponseDto {
  folders: FolderDto[];
  flashcards: FlashcardDto[];
}
