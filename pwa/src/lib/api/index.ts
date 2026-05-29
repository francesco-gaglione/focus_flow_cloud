// Barrel export — all consumers keep importing from "$lib/api"
export { ApiError, setTokens, clearTokens, getAccessToken, getRefreshToken } from "./client";
export { auth } from "./auth";
export { tasks } from "./tasks";
export { categories } from "./categories";
export { statsApi } from "./stats";
export { users } from "./users";
