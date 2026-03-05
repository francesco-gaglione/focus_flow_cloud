import 'package:equatable/equatable.dart';
import 'package:focus_flow_app/domain/entities/category_with_tasks.dart';
import 'package:focus_flow_app/domain/entities/task.dart';

enum CalendarViewMode { daily, weekly, monthly }

class CalendarState extends Equatable {
  final CalendarViewMode viewMode;
  final DateTime focusedDate;
  final DateTime? selectedDay;
  final List<Task> tasks;
  final List<CategoryWithTasks> categories;
  final bool isLoading;
  final String? error;

  const CalendarState({
    this.viewMode = CalendarViewMode.monthly,
    required this.focusedDate,
    this.selectedDay,
    this.tasks = const [],
    this.categories = const [],
    this.isLoading = false,
    this.error,
  });

  factory CalendarState.initial() => CalendarState(
    focusedDate: DateTime.now(),
    selectedDay: DateTime.now(),
  );

  /// Returns tasks that fall within the given date range (inclusive).
  List<Task> tasksForDateRange(DateTime from, DateTime to) {
    return tasks.where((task) {
      if (task.scheduledDate == null) return false;
      final taskDate = DateTime.fromMillisecondsSinceEpoch(
        task.scheduledDate! * 1000,
      );
      return !taskDate.isBefore(from) && taskDate.isBefore(to.add(const Duration(days: 1)));
    }).toList();
  }

  /// Returns tasks scheduled for a specific day.
  List<Task> tasksForDay(DateTime day) {
    final start = DateTime(day.year, day.month, day.day);
    final end = start.add(const Duration(days: 1));
    return tasks.where((task) {
      if (task.scheduledDate == null) return false;
      final taskDate = DateTime.fromMillisecondsSinceEpoch(
        task.scheduledDate! * 1000,
      );
      return taskDate.isAfter(start.subtract(const Duration(seconds: 1))) &&
          taskDate.isBefore(end);
    }).toList();
  }

  CalendarState copyWith({
    CalendarViewMode? viewMode,
    DateTime? focusedDate,
    DateTime? selectedDay,
    List<Task>? tasks,
    List<CategoryWithTasks>? categories,
    bool? isLoading,
    String? error,
    bool clearError = false,
  }) {
    return CalendarState(
      viewMode: viewMode ?? this.viewMode,
      focusedDate: focusedDate ?? this.focusedDate,
      selectedDay: selectedDay ?? this.selectedDay,
      tasks: tasks ?? this.tasks,
      categories: categories ?? this.categories,
      isLoading: isLoading ?? this.isLoading,
      error: clearError ? null : error ?? this.error,
    );
  }

  @override
  List<Object?> get props => [
    viewMode,
    focusedDate,
    selectedDay,
    tasks,
    categories,
    isLoading,
    error,
  ];
}
