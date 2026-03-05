# Focus Flow Cloud - Project Memory

## Architecture
- **Backend**: Rust (not to be touched)
- **Frontend**: Flutter app in `/app`
- **Pattern**: Clean architecture - entities → repositories → use cases → bloc/cubit → UI

## Project Structure (Flutter app)
```
lib/
  adapters/
    dtos/          # Freezed DTOs (run build_runner after changes)
    repositories/  # HTTP implementations
    ws/            # WebSocket
  domain/
    entities/      # Core business models
    repositories/  # Interfaces
    usecases/      # Use case classes
  presentation/
    app/           # Router, layout, app_view
    focus/         # Focus page + FocusBloc
    calendar/      # Calendar page + CalendarBloc (NEW)
    category/      # Category page + CategoryBloc
    ...
  core/
    di/            # GetIt service locator (service_locator.dart)
```

## Key Patterns
- **State management**: BLoC (flutter_bloc)
- **DI**: GetIt (`sl` instance in service_locator.dart)
- **Navigation**: go_router with ShellRoute for main layout
- **Serialization**: Freezed + json_serializable → run `dart run build_runner build --delete-conflicting-outputs`
- **Localization**: easy_localization, files in `assets/translations/en.yaml` and `it.yaml`

## Task Entity
- `scheduledDate`: Unix timestamp (seconds) — start time of scheduled task
- `scheduledEndDate`: Unix timestamp (seconds) — end time of scheduled task (NEW)
- Both optional; `isScheduled` getter = scheduledDate != null

## Key APIs
- `GET /api/task/scheduled?from=&to=&completed=` → ScheduledTasksResponseDto
- `PUT /api/task/{id}` → `{success: bool}` (NOT the updated task — construct locally)
- All timestamps are Unix seconds (not milliseconds)

## Calendar Feature (branch: 64-feat-implement-scheduled-tasks-and-calendar-view-dailyweeklymonthly)
- New route: `/calendar` → CalendarPage → CalendarBloc
- CalendarBloc in `presentation/calendar/bloc/`
- 3 views: Daily (time grid), Weekly (7-column), Monthly (grid with dots)
- Tap empty slot in daily view to create task
- Long press day in monthly view to create task
- Selecting a day in monthly view switches to daily view

## Focus Timeline Integration
- `FocusTimelineWidget` now accepts `scheduledTasks` + `onScheduledTaskTap` params
- Scheduled tasks shown as semi-transparent ghost items (opacity 0.55)
- Clicking a ghost item selects it as the active task + sets category in FocusBloc
- FocusBloc loads today's scheduled tasks via `ReloadTodayScheduledTasks` event

## Navigation
- Desktop: NavigationRail (5 items + settings trailing)
- Mobile: BottomNav (6 items: Focus, Categories, Calendar, Stats, Notes, Settings)
- Rail index: 0=focus, 1=categories, 2=calendar, 3=stats, 4=notes (settings=trailing)
