import 'package:equatable/equatable.dart';
import 'package:focus_flow_app/domain/entities/note_template.dart';

class SettingsState extends Equatable {
  final bool isLoading;
  final List<NoteTemplate> noteTemplates;
  final String? errorMessage;

  const SettingsState({
    this.isLoading = false,
    this.noteTemplates = const [],
    this.errorMessage,
  });

  SettingsState copyWith({
    bool? isLoading,
    List<NoteTemplate>? noteTemplates,
    String? errorMessage,
  }) {
    return SettingsState(
      isLoading: isLoading ?? this.isLoading,
      noteTemplates: noteTemplates ?? this.noteTemplates,
      errorMessage: errorMessage,
    );
  }

  @override
  List<Object?> get props => [isLoading, noteTemplates, errorMessage];
}
