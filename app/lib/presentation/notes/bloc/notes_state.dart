import 'package:equatable/equatable.dart';
import 'package:focus_flow_app/domain/entities/focus_session.dart';
import 'package:focus_flow_app/domain/entities/category.dart';

enum NotesStatus { initial, loading, success, failure }

class NotesState extends Equatable {
  final NotesStatus status;
  final List<FocusSession> sessions;
  final List<Category> categories;
  final DateTime? startDate;
  final DateTime? endDate;
  final String? selectedCategoryId;
  final String? errorMessage;
  final bool isUpdating;

  const NotesState({
    this.status = NotesStatus.initial,
    this.sessions = const [],
    this.categories = const [],
    this.startDate,
    this.endDate,
    this.selectedCategoryId,
    this.errorMessage,
    this.isUpdating = false,
  });

  NotesState copyWith({
    NotesStatus? status,
    List<FocusSession>? sessions,
    List<Category>? categories,
    DateTime? startDate,
    DateTime? endDate,
    String? selectedCategoryId,
    String? errorMessage,
    bool? isUpdating,
  }) {
    return NotesState(
      status: status ?? this.status,
      sessions: sessions ?? this.sessions,
      categories: categories ?? this.categories,
      startDate: startDate ?? this.startDate,
      endDate: endDate ?? this.endDate,
      selectedCategoryId: selectedCategoryId ?? this.selectedCategoryId,
      errorMessage: errorMessage ?? this.errorMessage,
      isUpdating: isUpdating ?? this.isUpdating,
    );
  }

  @override
  List<Object?> get props => [
        status,
        sessions,
        categories,
        startDate,
        endDate,
        selectedCategoryId,
        errorMessage,
        isUpdating,
      ];
}
