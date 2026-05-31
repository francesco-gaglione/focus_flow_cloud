import { apiClient } from "./client";
import type { GetStatsResponseDto } from "@/types";

export const statsApi = {
    get: () => {
        const tzOffset = new Date().getTimezoneOffset();
        return apiClient
            .get<GetStatsResponseDto>("/api/stats", { params: { tz_offset: tzOffset } })
            .then((r) => r.data);
    },
};
