

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/domain/usecases/sessions_usecases/get_sessions_with_filters.dart';
import 'package:focus_flow_app/domain/usecases/sessions_usecases/update_session.dart';
import 'package:focus_flow_app/domain/usecases/categories_usecases/get_categories_and_tasks.dart';
import 'package:focus_flow_app/domain/entities/focus_session.dart';
import 'package:logger/logger.dart';

import 'notes_event.dart';
import 'notes_state.dart';

class NotesBloc extends Bloc<NotesEvent, NotesState> {
  final logger = Logger();
  final GetSessionsWithFilters getSessionsWithFilters;
  final UpdateSession updateSession;
  final GetCategoriesAndTasks getCategoriesAndTasks;

  NotesBloc({
    required this.getSessionsWithFilters,
    required this.updateSession,
    required this.getCategoriesAndTasks,
  }) : super(const NotesState()) {
    on<NotesStarted>(_onNotesStarted);
    on<NotesDateRangeChanged>(_onDateRangeChanged);
    on<NotesCategoryChanged>(_onCategoryChanged);
    on<NotesFilterCleared>(_onFilterCleared);
    on<NotesNoteSaved>(_onNoteSaved);
  }

  Future<void> _onFilterCleared(
    NotesFilterCleared event,
    Emitter<NotesState> emit,
  ) async {
    final newState = NotesState(
      status: state.status,
      sessions: state.sessions,
      categories: state.categories,
      startDate: null,
      endDate: null,
      selectedCategoryId: null,
      errorMessage: state.errorMessage,
      isUpdating: state.isUpdating,
    );
    emit(newState);
    await _fetchNotes(emit, newState);
  }

  Future<void> _onNotesStarted(
    NotesStarted event,
    Emitter<NotesState> emit,
  ) async {
    await _fetchCategories(emit);
    await _fetchNotes(emit, state);
  }

  Future<void> _onDateRangeChanged(
    NotesDateRangeChanged event,
    Emitter<NotesState> emit,
  ) async {
    final newState = NotesState(
      status: state.status,
      sessions: state.sessions,
      categories: state.categories,
      startDate: event.startDate,
      endDate: event.endDate,
      selectedCategoryId: state.selectedCategoryId,
      errorMessage: state.errorMessage,
      isUpdating: state.isUpdating,
    );
    emit(newState);
    await _fetchNotes(emit, newState);
  }

  Future<void> _onCategoryChanged(
    NotesCategoryChanged event,
    Emitter<NotesState> emit,
  ) async {
    logger.i('Category changed to ${event.categoryId}');
    final newState = NotesState(
      status: state.status,
      sessions: state.sessions,
      categories: state.categories,
      startDate: state.startDate,
      endDate: state.endDate,
      selectedCategoryId: event.categoryId, // Allow null here
      errorMessage: state.errorMessage,
      isUpdating: state.isUpdating,
    );
    emit(newState);
    await _fetchNotes(emit, newState);
  }

  Future<void> _onNoteSaved(
    NotesNoteSaved event,
    Emitter<NotesState> emit,
  ) async {
    emit(state.copyWith(isUpdating: true));

    final result = await updateSession.execute(
      id: event.sessionId,
      notes: event.content,
    );

    if (result.success) {
      final updatedSessions =
          state.sessions.map((s) {
            if (s.id == event.sessionId) {
              return s.copyWith(notes: event.content);
            }
            return s;
          }).toList();

      emit(state.copyWith(isUpdating: false, sessions: updatedSessions));
    } else {
      emit(state.copyWith(isUpdating: false, errorMessage: result.error));
    }
  }

  Future<void> _fetchCategories(Emitter<NotesState> emit) async {
    final result = await getCategoriesAndTasks.execute();
    if (result.success && result.categoriesWithTasks != null) {
      final categories =
          result.categoriesWithTasks!.map((e) => e.category).toList();
      emit(state.copyWith(categories: categories));
    }
  }

  Future<void> _fetchNotes(
    Emitter<NotesState> emit,
    NotesState currentState,
  ) async {
    emit(currentState.copyWith(status: NotesStatus.loading));

    final result = await getSessionsWithFilters.execute(
      startDate: currentState.startDate != null
          ? currentState.startDate!.millisecondsSinceEpoch ~/ 1000
          : null,
      endDate: currentState.endDate != null
          ? DateTime(
            currentState.endDate!.year,
            currentState.endDate!.month,
            currentState.endDate!.day,
            23,
            59,
            59,
          ).millisecondsSinceEpoch ~/
          1000
          : null,
      categoryIds:
          currentState.selectedCategoryId != null
              ? [currentState.selectedCategoryId!]
              : null,
      hasNote: true,
    );

    if (result.success) {
      final sortedSessions = List<FocusSession>.from(result.sessions!)
        ..sort((a, b) => b.createdAt.compareTo(a.createdAt));

      emit(
        currentState.copyWith(
          status: NotesStatus.success,
          sessions: sortedSessions,
        ),
      );
    } else {
      emit(
        currentState.copyWith(
          status: NotesStatus.failure,
          errorMessage: result.error,
        ),
      );
    }
  }
}
