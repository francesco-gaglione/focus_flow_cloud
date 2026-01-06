import 'package:equatable/equatable.dart';

sealed class NotesEvent extends Equatable {
  const NotesEvent();

  @override
  List<Object?> get props => [];
}

class NotesStarted extends NotesEvent {}

class NotesDateRangeChanged extends NotesEvent {
  final DateTime? startDate;
  final DateTime? endDate;

  const NotesDateRangeChanged(this.startDate, this.endDate);

  @override
  List<Object?> get props => [startDate, endDate];
}

class NotesCategoryChanged extends NotesEvent {
  final String? categoryId;

  const NotesCategoryChanged(this.categoryId);

  @override
  List<Object?> get props => [categoryId];
}


class NotesFilterCleared extends NotesEvent {}

class NotesNoteSaved extends NotesEvent {

  final String sessionId;
  final String content;

  const NotesNoteSaved(this.sessionId, this.content);

  @override
  List<Object?> get props => [sessionId, content];
}
