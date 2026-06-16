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
    getAll: (
        completed: boolean,
        today: boolean,
        next_week: boolean,
        unscheduled: boolean,
        incoming: boolean,
        overdue: boolean = true,
    ) => {
        return apiClient
            .get<TasksResponseDto>("/api/task", {
                params: {
                    completed,
                    today,
                    nextWeek: next_week,
                    unscheduled,
                    incoming,
                    overdue,
                },
            })
            .then((r) => r.data);
    },

    create: (dto: CreateTaskDto) =>
        apiClient
            .post<CreateTaskResponseDto>("/api/task", dto)
            .then((r) => r.data),

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

    deleteSubtask: (taskId: string, subtaskId: string) =>
        apiClient
            .delete(`/api/task/${taskId}/subtask/${subtaskId}`)
            .then((r) => r.data),

    addReminder: (taskId: string, dto: { date: number }) =>
        apiClient
            .post(`/api/task/${taskId}/reminder`, dto)
            .then((r) => r.data),

    deleteReminder: (taskId: string, reminderId: string) =>
        apiClient
            .delete(`/api/task/${taskId}/reminder/${reminderId}`)
            .then((r) => r.data),
};
