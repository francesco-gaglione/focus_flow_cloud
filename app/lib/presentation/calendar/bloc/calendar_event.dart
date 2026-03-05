import 'package:focus_flow_app/presentation/calendar/bloc/calendar_state.dart';

abstract class CalendarEvent {}

class LoadCalendarData extends CalendarEvent {}

class ChangeViewMode extends CalendarEvent {
  final CalendarViewMode mode;
  ChangeViewMode(this.mode);
}

class NavigateDate extends CalendarEvent {
  final DateTime date;
  NavigateDate(this.date);
}

class NavigatePrevious extends CalendarEvent {}

class NavigateNext extends CalendarEvent {}

class NavigateToToday extends CalendarEvent {}

class CreateScheduledTask extends CalendarEvent {
  final String name;
  final String? description;
  final String? categoryId;
  final int scheduledDate;
  final int? scheduledEndDate;

  CreateScheduledTask({
    required this.name,
    this.description,
    this.categoryId,
    required this.scheduledDate,
    this.scheduledEndDate,
  });
}

class UpdateScheduledTask extends CalendarEvent {
  final String id;
  final String? name;
  final String? description;
  final String? categoryId;
  final int? scheduledDate;
  final int? scheduledEndDate;

  UpdateScheduledTask({
    required this.id,
    this.name,
    this.description,
    this.categoryId,
    this.scheduledDate,
    this.scheduledEndDate,
  });
}

class DeleteScheduledTask extends CalendarEvent {
  final String taskId;
  DeleteScheduledTask(this.taskId);
}

class SelectDay extends CalendarEvent {
  final DateTime day;
  SelectDay(this.day);
}

class CompleteScheduledTask extends CalendarEvent {
  final String taskId;
  CompleteScheduledTask(this.taskId);
}

class UncompleteScheduledTask extends CalendarEvent {
  final String taskId;
  UncompleteScheduledTask(this.taskId);
}
