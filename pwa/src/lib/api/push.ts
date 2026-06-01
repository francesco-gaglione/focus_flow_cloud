import { apiClient } from "./client";

export const pushApi = {
    subscribe: (subscription: PushSubscriptionJSON) =>
        apiClient.post("/api/push-subscriptions", subscription).then((r) => r.data),
};
