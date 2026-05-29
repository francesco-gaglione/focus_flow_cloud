import { apiClient } from "./client";
import type {
    TasksResponseDto,
    CreateTaskDto,
    CreateTaskResponseDto,
    UpdateTaskDto,
    UpdateTaskResponseDto,
    DeleteTaskResponseDto,
    CreateSubtaskDto,
    CreateSubtaskResponseDto,
    UpdateSubTaskDto,
    UpdateSubTaskResponseDto,
} from "@/types";

export const tasks = {
    getAll: (completed?: boolean | any) => {
        const isBool = typeof completed === "boolean";
        return apiClient
            .get<TasksResponseDto>("/api/task", {
                params: isBool ? { completed } : undefined,
            })
            .then((r) => r.data);
    },

    create: (dto: CreateTaskDto) =>
        apiClient.post<CreateTaskResponseDto>("/api/task", dto).then((r) => r.data),

    update: (id: string, dto: UpdateTaskDto) =>
        apiClient
            .patch<UpdateTaskResponseDto>(`/api/task/${id}`, dto)
            .then((r) => r.data),

    delete: (taskId: string) =>
        apiClient
            .delete<DeleteTaskResponseDto>("/api/task", { params: { taskId } })
            .then((r) => r.data),

    createSubtask: (taskId: string, dto: CreateSubtaskDto) =>
        apiClient
            .post<CreateSubtaskResponseDto>(`/api/task/${taskId}/subtask`, dto)
            .then((r) => r.data),

    updateSubtask: (taskId: string, subtaskId: string, dto: UpdateSubTaskDto) =>
        apiClient
            .patch<UpdateSubTaskResponseDto>(
                `/api/task/${taskId}/subtask/${subtaskId}`,
                dto,
            )
            .then((r) => r.data),
};
