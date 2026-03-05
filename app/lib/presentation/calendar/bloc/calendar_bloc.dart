import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/domain/usecases/categories_usecases/get_categories_and_tasks.dart';
import 'package:focus_flow_app/domain/usecases/tasks_usecases/complete_task.dart';
import 'package:focus_flow_app/domain/usecases/tasks_usecases/create_task.dart';
import 'package:focus_flow_app/domain/usecases/tasks_usecases/delete_tasks.dart';
import 'package:focus_flow_app/domain/usecases/tasks_usecases/get_scheduled_tasks.dart';
import 'package:focus_flow_app/domain/usecases/tasks_usecases/uncomplete_task.dart';
import 'package:focus_flow_app/domain/usecases/tasks_usecases/update_task.dart';
import 'package:focus_flow_app/presentation/calendar/bloc/calendar_event.dart';
import 'package:focus_flow_app/presentation/calendar/bloc/calendar_state.dart';
import 'package:logger/logger.dart';

class CalendarBloc extends Bloc<CalendarEvent, CalendarState> {
  final Logger _logger = Logger();
  final GetScheduledTasks _getScheduledTasks;
  final GetCategoriesAndTasks _getCategoriesAndTasks;
  final CreateTask _createTask;
  final UpdateTask _updateTask;
  final DeleteTasks _deleteTasks;
  final CompleteTask _completeTask;
  final UncompleteTask _uncompleteTask;

  CalendarBloc({
    required GetScheduledTasks getScheduledTasks,
    required GetCategoriesAndTasks getCategoriesAndTasks,
    required CreateTask createTask,
    required UpdateTask updateTask,
    required DeleteTasks deleteTasks,
    required CompleteTask completeTask,
    required UncompleteTask uncompleteTask,
  })  : _getScheduledTasks = getScheduledTasks,
        _getCategoriesAndTasks = getCategoriesAndTasks,
        _createTask = createTask,
        _updateTask = updateTask,
        _deleteTasks = deleteTasks,
        _completeTask = completeTask,
        _uncompleteTask = uncompleteTask,
        super(CalendarState.initial()) {
    on<LoadCalendarData>(_onLoadCalendarData);
    on<ChangeViewMode>(_onChangeViewMode);
    on<NavigateDate>(_onNavigateDate);
    on<NavigatePrevious>(_onNavigatePrevious);
    on<NavigateNext>(_onNavigateNext);
    on<NavigateToToday>(_onNavigateToToday);
    on<CreateScheduledTask>(_onCreateScheduledTask);
    on<UpdateScheduledTask>(_onUpdateScheduledTask);
    on<DeleteScheduledTask>(_onDeleteScheduledTask);
    on<SelectDay>(_onSelectDay);
    on<CompleteScheduledTask>(_onCompleteScheduledTask);
    on<UncompleteScheduledTask>(_onUncompleteScheduledTask);
  }

  /// Compute the date range for loading tasks based on current view mode.
  ({DateTime from, DateTime to}) _getLoadRange(CalendarState s) {
    final d = s.focusedDate;
    switch (s.viewMode) {
      case CalendarViewMode.daily:
        final start = DateTime(d.year, d.month, d.day);
        return (from: start, to: start.add(const Duration(days: 1)));
      case CalendarViewMode.weekly:
        final monday = d.subtract(Duration(days: d.weekday - 1));
        final start = DateTime(monday.year, monday.month, monday.day);
        return (from: start, to: start.add(const Duration(days: 7)));
      case CalendarViewMode.monthly:
        // Load ±1 month for smooth navigation
        final start = DateTime(d.year, d.month - 1, 1);
        final end = DateTime(d.year, d.month + 2, 1);
        return (from: start, to: end);
    }
  }

  Future<void> _loadData(Emitter<CalendarState> emit) async {
    emit(state.copyWith(isLoading: true, clearError: true));
    try {
      final range = _getLoadRange(state);
      final results = await Future.wait([
        _getScheduledTasks.execute(
          from: range.from.millisecondsSinceEpoch ~/ 1000,
          to: range.to.millisecondsSinceEpoch ~/ 1000,
        ),
        _getCategoriesAndTasks.execute(),
      ]);

      final tasksResult = results[0] as GetScheduledTasksResult;
      final categoriesResult = results[1] as GetCategoriesAndTasksResult;

      emit(
        state.copyWith(
          tasks: tasksResult.tasks,
          categories: categoriesResult.categoriesWithTasks ?? [],
          isLoading: false,
        ),
      );
    } catch (e) {
      _logger.e('CalendarBloc error loading data', error: e);
      emit(state.copyWith(isLoading: false, error: e.toString()));
    }
  }

  Future<void> _onLoadCalendarData(
    LoadCalendarData event,
    Emitter<CalendarState> emit,
  ) async {
    await _loadData(emit);
  }

  Future<void> _onChangeViewMode(
    ChangeViewMode event,
    Emitter<CalendarState> emit,
  ) async {
    emit(state.copyWith(viewMode: event.mode));
    await _loadData(emit);
  }

  Future<void> _onNavigateDate(
    NavigateDate event,
    Emitter<CalendarState> emit,
  ) async {
    emit(state.copyWith(focusedDate: event.date, selectedDay: event.date));
    await _loadData(emit);
  }

  Future<void> _onNavigatePrevious(
    NavigatePrevious event,
    Emitter<CalendarState> emit,
  ) async {
    final d = state.focusedDate;
    DateTime newDate;
    switch (state.viewMode) {
      case CalendarViewMode.daily:
        newDate = d.subtract(const Duration(days: 1));
        break;
      case CalendarViewMode.weekly:
        newDate = d.subtract(const Duration(days: 7));
        break;
      case CalendarViewMode.monthly:
        newDate = DateTime(d.year, d.month - 1, 1);
        break;
    }
    emit(state.copyWith(focusedDate: newDate));
    await _loadData(emit);
  }

  Future<void> _onNavigateNext(
    NavigateNext event,
    Emitter<CalendarState> emit,
  ) async {
    final d = state.focusedDate;
    DateTime newDate;
    switch (state.viewMode) {
      case CalendarViewMode.daily:
        newDate = d.add(const Duration(days: 1));
        break;
      case CalendarViewMode.weekly:
        newDate = d.add(const Duration(days: 7));
        break;
      case CalendarViewMode.monthly:
        newDate = DateTime(d.year, d.month + 1, 1);
        break;
    }
    emit(state.copyWith(focusedDate: newDate));
    await _loadData(emit);
  }

  Future<void> _onNavigateToToday(
    NavigateToToday event,
    Emitter<CalendarState> emit,
  ) async {
    final today = DateTime.now();
    emit(state.copyWith(focusedDate: today, selectedDay: today));
    await _loadData(emit);
  }

  Future<void> _onCreateScheduledTask(
    CreateScheduledTask event,
    Emitter<CalendarState> emit,
  ) async {
    try {
      final result = await _createTask.execute(
        name: event.name,
        description: event.description,
        categoryId: event.categoryId,
        scheduledDate: event.scheduledDate,
        scheduledEndDate: event.scheduledEndDate,
      );
      if (result.success) {
        await _loadData(emit);
      } else {
        emit(state.copyWith(error: result.error));
      }
    } catch (e) {
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _onUpdateScheduledTask(
    UpdateScheduledTask event,
    Emitter<CalendarState> emit,
  ) async {
    try {
      final result = await _updateTask.execute(
        id: event.id,
        name: event.name,
        description: event.description,
        categoryId: event.categoryId,
        scheduledDate: event.scheduledDate,
        scheduledEndDate: event.scheduledEndDate,
      );
      if (result.success) {
        await _loadData(emit);
      } else {
        emit(state.copyWith(error: result.error));
      }
    } catch (e) {
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _onDeleteScheduledTask(
    DeleteScheduledTask event,
    Emitter<CalendarState> emit,
  ) async {
    try {
      final result = await _deleteTasks.execute(taskIds: [event.taskId]);
      if (result.success) {
        await _loadData(emit);
      } else {
        emit(state.copyWith(error: result.error));
      }
    } catch (e) {
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _onSelectDay(
    SelectDay event,
    Emitter<CalendarState> emit,
  ) async {
    emit(
      state.copyWith(
        selectedDay: event.day,
        focusedDate: event.day,
        viewMode: CalendarViewMode.daily,
      ),
    );
    await _loadData(emit);
  }

  Future<void> _onCompleteScheduledTask(
    CompleteScheduledTask event,
    Emitter<CalendarState> emit,
  ) async {
    try {
      final result = await _completeTask.execute(taskId: event.taskId);
      if (result.success) {
        await _loadData(emit);
      } else {
        emit(state.copyWith(error: result.error));
      }
    } catch (e) {
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _onUncompleteScheduledTask(
    UncompleteScheduledTask event,
    Emitter<CalendarState> emit,
  ) async {
    try {
      final result = await _uncompleteTask.execute(taskId: event.taskId);
      if (result.success) {
        await _loadData(emit);
      } else {
        emit(state.copyWith(error: result.error));
      }
    } catch (e) {
      emit(state.copyWith(error: e.toString()));
    }
  }
}
