import 'package:focus_flow_app/domain/entities/category.dart';
import 'package:focus_flow_app/domain/entities/task.dart';

abstract class FocusEvent {}

class InitState extends FocusEvent {}

class CategorySelected extends FocusEvent {
  final Category? category;

  CategorySelected({required this.category});
}

class TaskSelected extends FocusEvent {
  final Task? task;

  TaskSelected({required this.task});
}

class StartFocus extends FocusEvent {}

class BreakFocus extends FocusEvent {}

class TerminateFocus extends FocusEvent {}

class UpdateNote extends FocusEvent {
  final String note;

  UpdateNote({required this.note});
}

class FocusLevelSelected extends FocusEvent {
  final int focusLevel;

  FocusLevelSelected({required this.focusLevel});
}

class PomodoroStateUpdated extends FocusEvent {
  final dynamic pomodoroState;

  PomodoroStateUpdated(this.pomodoroState);
}

class ReloadTodaySessions extends FocusEvent {}

class ReloadCategoriesAndTasks extends FocusEvent {}

class WebSocketConnectionUpdated extends FocusEvent {
  final bool isConnected;

  WebSocketConnectionUpdated(this.isConnected);
}

class CheckConnection extends FocusEvent {}

class AddManualSession extends FocusEvent {
  final Category? category;
  final Task? task;
  final DateTime startTime;
  final DateTime endTime;
  final int focusLevel;
  final String? note;

  AddManualSession({
    this.category,
    this.task,
    required this.startTime,
    required this.endTime,
    required this.focusLevel,
    this.note,
  });
}
