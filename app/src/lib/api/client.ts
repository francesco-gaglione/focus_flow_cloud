import axios, { AxiosError } from "axios";
import type { RefreshResponseDto } from "@/types";

// ── Custom error ─────────────────────────────────────────────────
export class ApiError extends Error {
    status: number;
    constructor(status: number, message: string) {
        super(message);
        this.name = "ApiError";
        this.status = status;
    }
    get isUnauthorized() {
        return this.status === 401;
    }
}

// ── Token management ─────────────────────────────────────────────
const LS_ACCESS = "ff_access";
const LS_REFRESH = "ff_refresh";

let accessToken: string | null = localStorage.getItem(LS_ACCESS);
let refreshToken: string | null = localStorage.getItem(LS_REFRESH);

export function setTokens(access: string, refresh: string) {
    accessToken = access;
    refreshToken = refresh;
    localStorage.setItem(LS_ACCESS, access);
    localStorage.setItem(LS_REFRESH, refresh);
}

export function clearTokens() {
    accessToken = null;
    refreshToken = null;
    localStorage.removeItem(LS_ACCESS);
    localStorage.removeItem(LS_REFRESH);
}

export function getAccessToken() {
    return accessToken;
}

export function getRefreshToken() {
    return refreshToken;
}

// ── Axios instance ───────────────────────────────────────────────
const BASE_URL = import.meta.env.VITE_API_BASE_URL || "";

export const apiClient = axios.create({
    baseURL: BASE_URL,
    headers: {
        "Content-Type": "application/json",
        Accept: "application/json",
    },
});

// Request interceptor — attach access token
apiClient.interceptors.request.use((config) => {
    if (accessToken) {
        config.headers.Authorization = `Bearer ${accessToken}`;
    }
    return config;
});

// Response interceptor — auto-refresh on 401
let refreshPromise: Promise<boolean> | null = null;

async function tryRefresh(): Promise<boolean> {
    if (!refreshToken) return false;
    try {
        // Use raw axios to avoid interceptor loops
        const res = await axios.post<RefreshResponseDto>(
            `${BASE_URL}/api/auth/refresh`,
            { refreshToken },
            { headers: { "Content-Type": "application/json" } },
        );
        setTokens(res.data.token, res.data.refreshToken);
        return true;
    } catch {
        return false;
    }
}

apiClient.interceptors.response.use(
    (response) => response,
    async (error: AxiosError) => {
        const original = error.config;
        if (
            error.response?.status === 401 &&
            original &&
            !(original as any).__isRetry &&
            refreshToken
        ) {
            (original as any).__isRetry = true;

            // Deduplicate concurrent refresh attempts
            if (!refreshPromise) {
                refreshPromise = tryRefresh().finally(() => {
                    refreshPromise = null;
                });
            }

            const ok = await refreshPromise;
            if (ok) {
                original.headers.Authorization = `Bearer ${accessToken}`;
                return apiClient(original);
            }
        }

        // Convert to ApiError for backward compatibility
        const status = error.response?.status ?? 0;
        const message =
            typeof error.response?.data === "string"
                ? error.response.data
                : error.message;
        return Promise.reject(new ApiError(status, message));
    },
);
