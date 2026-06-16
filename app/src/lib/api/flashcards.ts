import { apiClient } from "./client";
import type {
    RootFolderContentsResponseDto,
    FolderContentsResponseDto,
    DueFlashcardsResponseDto,
    CreateFlashcardDto,
    UpdateFlashcardDto,
    ReviewFlashcardDto,
    CreateFolderDto,
    FlashcardGlobalStatsDto,
    ActivityHeatmapResponseDto,
    FlashcardFolderStatsDto,
    FolderReviewQueueResponseDto,
} from "@/types";

export const flashcardsApi = {
    getRootFolderContents: () =>
        apiClient
            .get<RootFolderContentsResponseDto>("/api/flashcard/folder/root/contents")
            .then((r) => r.data),

    getFolderContents: (folderId: string) =>
        apiClient
            .get<FolderContentsResponseDto>(`/api/flashcard/folder/${folderId}/contents`)
            .then((r) => r.data),

    getDueFlashcards: () =>
        apiClient
            .get<DueFlashcardsResponseDto>("/api/flashcard/due")
            .then((r) => r.data),

    createFlashcard: (dto: CreateFlashcardDto) =>
        apiClient
            .post<void>("/api/flashcard", dto)
            .then((r) => r.data),

    updateFlashcard: (id: string, dto: UpdateFlashcardDto) =>
        apiClient
            .put<void>(`/api/flashcard/${id}`, dto)
            .then((r) => r.data),

    deleteFlashcard: (id: string) =>
        apiClient
            .delete<void>(`/api/flashcard/${id}`)
            .then((r) => r.data),

    reviewFlashcard: (id: string, dto: ReviewFlashcardDto) =>
        apiClient
            .post<void>(`/api/flashcard/${id}/review`, dto)
            .then((r) => r.data),

    createFolder: (dto: CreateFolderDto) =>
        apiClient
            .post<{ id: string; name: string }>("/api/flashcard/folder", dto)
            .then((r) => r.data),

    deleteFolder: (id: string) =>
        apiClient
            .delete<void>(`/api/flashcard/folder/${id}`)
            .then((r) => r.data),

    getGlobalStats: () =>
        apiClient
            .get<FlashcardGlobalStatsDto>("/api/flashcard/stats")
            .then((r) => r.data),

    getActivityHeatmap: (days?: number) =>
        apiClient
            .get<ActivityHeatmapResponseDto>("/api/flashcard/stats/activity", {
                params: days !== undefined ? { days } : undefined,
            })
            .then((r) => r.data),

    getFolderStats: (folderId: string) =>
        apiClient
            .get<FlashcardFolderStatsDto>(`/api/flashcard/folder/${folderId}/stats`)
            .then((r) => r.data),

    getFolderReviewQueue: (folderId: string) =>
        apiClient
            .get<FolderReviewQueueResponseDto>(`/api/flashcard/folder/${folderId}/review/queue`)
            .then((r) => r.data),
};
