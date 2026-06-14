export type CardRating = 'Again' | 'Hard' | 'Good' | 'Easy';

export interface ReviewFlashcardDto {
  rating: CardRating;
  elapsedDays: number;
}
