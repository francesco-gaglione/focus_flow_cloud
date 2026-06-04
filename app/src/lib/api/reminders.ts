import { apiClient } from "./client";

export interface PendingReminderDto {
    id: string;
    dateTime: number;
    title: string;
    description: string;
}

export const remindersApi = {
    getPending: (): Promise<PendingReminderDto[]> =>
        apiClient
            .get<{ reminders: PendingReminderDto[] }>("/api/reminders/pending")
            .then((r) => r.data.reminders),
};
