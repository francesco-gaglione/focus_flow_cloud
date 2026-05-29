// Auto-generated from Rust DTOs — run `npm run generate:types` to update

export type { LoginDto } from './generated/LoginDto'
export type { LoginResponseDto } from './generated/LoginResponseDto'
export type { RefreshDto } from './generated/RefreshDto'
export type { RefreshResponseDto } from './generated/RefreshResponseDto'
export type { LogoutResponseDto } from './generated/LogoutResponseDto'

export type { TaskDto } from './generated/TaskDto'
export type { SubtaskDto } from './generated/SubtaskDto'
export type { TaskPriority } from './generated/TaskPriority'
export type { TaskScheduleDto } from './generated/TaskScheduleDto'
export type { TasksResponseDto } from './generated/TasksResponseDto'
export type { CreateTaskDto } from './generated/CreateTaskDto'
export type { CreateTaskResponseDto } from './generated/CreateTaskResponseDto'
export type { UpdateTaskDto } from './generated/UpdateTaskDto'
export type { UpdateTaskResponseDto } from './generated/UpdateTaskResponseDto'
export type { DeleteTaskResponseDto } from './generated/DeleteTaskResponseDto'
export type { CreateSubtaskDto } from './generated/CreateSubtaskDto'
export type { CreateSubtaskResponseDto } from './generated/CreateSubtaskResponseDto'
export type { UpdateSubTaskDto } from './generated/UpdateSubTaskDto'
export type { UpdateSubTaskResponseDto } from './generated/UpdateSubTaskResponseDto'

export type { CategoryDto } from './generated/CategoryDto'
export type { GetAllCategoryResponseDto } from './generated/GetAllCategoryResponseDto'
export type { CreateCategoryDto } from './generated/CreateCategoryDto'
export type { CreateCategoryResponseDto } from './generated/CreateCategoryResponseDto'
export type { UpdateCategoryDto } from './generated/UpdateCategoryDto'
export type { UpdateCategoryResponseDto } from './generated/UpdateCategoryResponseDto'
export type { DeleteCategoriesDto } from './generated/DeleteCategoriesDto'
export type { DeleteCategoriesResponseDto } from './generated/DeleteCategoriesResponseDto'

export type { GetStatsResponseDto } from './generated/GetStatsResponseDto'
export type { CompletedTasksCountsDto } from './generated/CompletedTasksCountsDto'
export type { PeakWindowRangeDto } from './generated/PeakWindowRangeDto'
export type { CompletedByPriorityDto } from './generated/CompletedByPriorityDto'
export type { CompletedFocusSessionsDto } from './generated/CompletedFocusSessionsDto'
export type { OverdueTrendTypeDto } from './generated/OverdueTrendTypeDto'
export type { OverdueTrendDto } from './generated/OverdueTrendDto'
export type { CategoryCountDto } from './generated/CategoryCountDto'
export type { DayCountDto } from './generated/DayCountDto'
export type { WeekCountDto } from './generated/WeekCountDto'

export type { SessionTypeEnum } from './generated/SessionTypeEnum'
export type { FocusSessionDto } from './generated/FocusSessionDto'

export type { UserInfoDto } from './generated/UserInfoDto'
export type { CreateUserDto } from './generated/CreateUserDto'
export type { UpdateUsernameDto } from './generated/UpdateUsernameDto'
export type { UpdatePasswordDto } from './generated/UpdatePasswordDto'
export type { UserSettingDto } from './generated/UserSettingDto'

// WebSocket types (not in backend shared crate yet)
export type WsSessionType = 'Work' | 'ShortBreak' | 'LongBreak'

export interface WsCurrentSession {
  sessionType: WsSessionType
  sessionStartTime: number
  note: string | null
  concentrationScore: number | null
}

export interface PomodoroWsState {
  currentSession: WsCurrentSession | null
  taskId: string | null
}
