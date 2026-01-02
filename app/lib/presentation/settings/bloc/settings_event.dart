import 'package:equatable/equatable.dart';
import 'package:focus_flow_app/domain/entities/note_template.dart';

abstract class SettingsEvent extends Equatable {
  const SettingsEvent();

  @override
  List<Object?> get props => [];
}

class LoadSettings extends SettingsEvent {}

class AddNoteTemplate extends SettingsEvent {
  final NoteTemplate template;

  const AddNoteTemplate(this.template);

  @override
  List<Object?> get props => [template];
}

class EditNoteTemplate extends SettingsEvent {
  final NoteTemplate template;

  const EditNoteTemplate(this.template);

  @override
  List<Object?> get props => [template];
}

class DeleteNoteTemplate extends SettingsEvent {
  final String id;

  const DeleteNoteTemplate(this.id);

  @override
  List<Object?> get props => [id];
}
