import { apiClient } from "./client";
import type {
    UserInfoDto,
    CreateUserDto,
    UpdateUsernameDto,
    UpdatePasswordDto,
} from "@/types";

export const users = {
    me: () =>
        apiClient.get<UserInfoDto>("/api/users/me").then((r) => r.data),

    create: (dto: CreateUserDto) =>
        apiClient.post<void>("/api/users", dto).then((r) => r.data),

    updateUsername: (dto: UpdateUsernameDto) =>
        apiClient.put<void>("/api/users/username", dto).then((r) => r.data),

    updatePassword: (dto: UpdatePasswordDto) =>
        apiClient.put<void>("/api/users/password", dto).then((r) => r.data),
};
