import { apiClient } from "./client";
import type {
    GetAllCategoryResponseDto,
    CreateCategoryDto,
    CreateCategoryResponseDto,
    UpdateCategoryDto,
    UpdateCategoryResponseDto,
} from "@/types";

export const categories = {
    getAll: () =>
        apiClient
            .get<GetAllCategoryResponseDto>("/api/category/categories")
            .then((r) => r.data),

    create: (dto: CreateCategoryDto) =>
        apiClient
            .post<CreateCategoryResponseDto>("/api/category", dto)
            .then((r) => r.data),

    update: (id: string, dto: UpdateCategoryDto) =>
        apiClient
            .patch<UpdateCategoryResponseDto>(`/api/category/${id}`, dto)
            .then((r) => r.data),

    delete: (id: string) =>
        apiClient.delete<void>(`/api/category/${id}`).then((r) => r.data),
};
