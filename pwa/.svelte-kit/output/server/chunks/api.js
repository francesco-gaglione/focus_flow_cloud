import axios from "axios";
//#region src/lib/api/client.ts
var ApiError = class extends Error {
	status;
	constructor(status, message) {
		super(message);
		this.name = "ApiError";
		this.status = status;
	}
	get isUnauthorized() {
		return this.status === 401;
	}
};
var LS_ACCESS = "ff_access";
var LS_REFRESH = "ff_refresh";
var accessToken = localStorage.getItem(LS_ACCESS);
var refreshToken = localStorage.getItem(LS_REFRESH);
function setTokens(access, refresh) {
	accessToken = access;
	refreshToken = refresh;
	localStorage.setItem(LS_ACCESS, access);
	localStorage.setItem(LS_REFRESH, refresh);
}
function clearTokens() {
	accessToken = null;
	refreshToken = null;
	localStorage.removeItem(LS_ACCESS);
	localStorage.removeItem(LS_REFRESH);
}
function getAccessToken() {
	return accessToken;
}
var BASE_URL = "http://192.168.1.135:8080";
var apiClient = axios.create({
	baseURL: BASE_URL,
	headers: {
		"Content-Type": "application/json",
		Accept: "application/json"
	}
});
apiClient.interceptors.request.use((config) => {
	if (accessToken) config.headers.Authorization = `Bearer ${accessToken}`;
	return config;
});
var refreshPromise = null;
async function tryRefresh() {
	if (!refreshToken) return false;
	try {
		const res = await axios.post(`${BASE_URL}/api/auth/refresh`, { refreshToken }, { headers: { "Content-Type": "application/json" } });
		setTokens(res.data.token, res.data.refreshToken);
		return true;
	} catch {
		return false;
	}
}
apiClient.interceptors.response.use((response) => response, async (error) => {
	const original = error.config;
	if (error.response?.status === 401 && original && !original.__isRetry && refreshToken) {
		original.__isRetry = true;
		if (!refreshPromise) refreshPromise = tryRefresh().finally(() => {
			refreshPromise = null;
		});
		if (await refreshPromise) {
			original.headers.Authorization = `Bearer ${accessToken}`;
			return apiClient(original);
		}
	}
	const status = error.response?.status ?? 0;
	const message = typeof error.response?.data === "string" ? error.response.data : error.message;
	return Promise.reject(new ApiError(status, message));
});
//#endregion
//#region src/lib/api/tasks.ts
var tasks = {
	getAll: (completed) => {
		const isBool = typeof completed === "boolean";
		return apiClient.get("/api/task", { params: isBool ? { completed } : void 0 }).then((r) => r.data);
	},
	create: (dto) => apiClient.post("/api/task", dto).then((r) => r.data),
	update: (id, dto) => apiClient.patch(`/api/task/${id}`, dto).then((r) => r.data),
	delete: (taskId) => apiClient.delete("/api/task", { params: { taskId } }).then((r) => r.data),
	createSubtask: (taskId, dto) => apiClient.post(`/api/task/${taskId}/subtask`, dto).then((r) => r.data),
	updateSubtask: (taskId, subtaskId, dto) => apiClient.patch(`/api/task/${taskId}/subtask/${subtaskId}`, dto).then((r) => r.data)
};
//#endregion
//#region src/lib/api/categories.ts
var categories = {
	getAll: () => apiClient.get("/api/category/categories").then((r) => r.data),
	create: (dto) => apiClient.post("/api/category", dto).then((r) => r.data),
	update: (id, dto) => apiClient.patch(`/api/category/${id}`, dto).then((r) => r.data),
	delete: (id) => apiClient.delete(`/api/category/${id}`).then((r) => r.data)
};
//#endregion
//#region src/lib/api/stats.ts
var statsApi = { get: () => apiClient.get("/api/stats").then((r) => r.data) };
//#endregion
//#region src/lib/api/users.ts
var users = {
	me: () => apiClient.get("/api/users/me").then((r) => r.data),
	create: (dto) => apiClient.post("/api/users", dto).then((r) => r.data),
	updateUsername: (dto) => apiClient.put("/api/users/username", dto).then((r) => r.data),
	updatePassword: (dto) => apiClient.put("/api/users/password", dto).then((r) => r.data)
};
//#endregion
export { ApiError as a, setTokens as c, tasks as i, statsApi as n, clearTokens as o, categories as r, getAccessToken as s, users as t };
