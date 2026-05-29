import { apiClient } from "./client";
import type {
    LoginDto,
    LoginResponseDto,
    LogoutResponseDto,
    RefreshDto,
    RefreshResponseDto,
} from "@/types";

export const auth = {
    login: (dto: LoginDto) =>
        apiClient.post<LoginResponseDto>("/api/auth/login", dto).then((r) => r.data),

    logout: () =>
        apiClient.post<LogoutResponseDto>("/api/auth/logout").then((r) => r.data),

    refresh: (dto: RefreshDto) =>
        apiClient
            .post<RefreshResponseDto>("/api/auth/refresh", dto)
            .then((r) => r.data),
};
