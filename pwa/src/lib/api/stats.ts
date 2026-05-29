import { apiClient } from "./client";
import type { GetStatsResponseDto } from "@/types";

export const statsApi = {
    get: () =>
        apiClient.get<GetStatsResponseDto>("/api/stats").then((r) => r.data),
};
