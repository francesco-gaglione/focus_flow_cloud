import 'dart:convert';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/domain/entities/note_template.dart';
import 'package:focus_flow_app/domain/entities/user_setting.dart';
import 'package:focus_flow_app/domain/repositories/user_settings_repository.dart';
import 'package:focus_flow_app/presentation/settings/bloc/settings_event.dart';
import 'package:focus_flow_app/presentation/settings/bloc/settings_state.dart';

class SettingsBloc extends Bloc<SettingsEvent, SettingsState> {
  final UserSettingsRepository _userSettingsRepository;

  SettingsBloc({
    required UserSettingsRepository userSettingsRepository,
  }) : _userSettingsRepository = userSettingsRepository,
       super(const SettingsState()) {
    on<LoadSettings>(_onLoadSettings);
    on<AddNoteTemplate>(_onAddNoteTemplate);
    on<EditNoteTemplate>(_onEditNoteTemplate);
    on<DeleteNoteTemplate>(_onDeleteNoteTemplate);
  }

  Future<void> _onLoadSettings(
    LoadSettings event,
    Emitter<SettingsState> emit,
  ) async {
    emit(state.copyWith(isLoading: true, errorMessage: null));
    try {
      final settings = await _userSettingsRepository.getUserSettings();
      final templates = _parseTemplates(settings);
      emit(state.copyWith(isLoading: false, noteTemplates: templates));
    } catch (e) {
      emit(state.copyWith(isLoading: false, errorMessage: e.toString()));
    }
  }

  Future<void> _onAddNoteTemplate(
    AddNoteTemplate event,
    Emitter<SettingsState> emit,
  ) async {
    emit(state.copyWith(isLoading: true, errorMessage: null));
    try {
      final currentTemplates = List<NoteTemplate>.from(state.noteTemplates);
      currentTemplates.add(event.template);
      await _saveTemplates(currentTemplates);
      emit(state.copyWith(isLoading: false, noteTemplates: currentTemplates));
    } catch (e) {
      emit(state.copyWith(isLoading: false, errorMessage: e.toString()));
    }
  }

  Future<void> _onEditNoteTemplate(
    EditNoteTemplate event,
    Emitter<SettingsState> emit,
  ) async {
    emit(state.copyWith(isLoading: true, errorMessage: null));
    try {
      final currentTemplates = List<NoteTemplate>.from(state.noteTemplates);
      final index = currentTemplates.indexWhere((t) => t.id == event.template.id);
      if (index != -1) {
        currentTemplates[index] = event.template;
        await _saveTemplates(currentTemplates);
        emit(state.copyWith(isLoading: false, noteTemplates: currentTemplates));
      }
    } catch (e) {
      emit(state.copyWith(isLoading: false, errorMessage: e.toString()));
    }
  }

  Future<void> _onDeleteNoteTemplate(
    DeleteNoteTemplate event,
    Emitter<SettingsState> emit,
  ) async {
    emit(state.copyWith(isLoading: true, errorMessage: null));
    try {
      final currentTemplates = List<NoteTemplate>.from(state.noteTemplates);
      currentTemplates.removeWhere((t) => t.id == event.id);
      await _saveTemplates(currentTemplates);
      emit(state.copyWith(isLoading: false, noteTemplates: currentTemplates));
    } catch (e) {
      emit(state.copyWith(isLoading: false, errorMessage: e.toString()));
    }
  }

  Future<void> _saveTemplates(List<NoteTemplate> templates) async {
    await _userSettingsRepository.updateUserSetting(
      UserSetting(
        key: 'note_templates',
        value: jsonEncode(templates.map((t) => t.toJson()).toList()),
      ),
    );
  }

  List<NoteTemplate> _parseTemplates(List<UserSetting> settings) {
    try {
      final templatesJsonString =
          settings
              .firstWhere(
                (s) => s.key == 'note_templates',
                orElse: () => const UserSetting(key: 'note_templates', value: ''),
              )
              .value;

      if (templatesJsonString.isNotEmpty) {
        final List<dynamic> jsonList = jsonDecode(templatesJsonString);
        return jsonList.map((json) => NoteTemplate.fromJson(json)).toList();
      }
    } catch (_) {
      // Ignore errors
    }
    return [];
  }
}
