import 'package:flutter_bloc/flutter_bloc.dart';

import '../../domain/entities/theme_settings.dart';
import '../../domain/usecases/get_theme_settings.dart';
import '../../domain/usecases/toggle_theme.dart';
import '../../domain/usecases/update_accent_color.dart';

class ThemeState {
  final ThemeSettings settings;
  final bool isLoading;

  const ThemeState({required this.settings, this.isLoading = false});

  bool get isDarkMode => settings.isDarkMode;
  int get accentColor => settings.accentColor;

  ThemeState copyWith({ThemeSettings? settings, bool? isLoading}) {
    return ThemeState(
      settings: settings ?? this.settings,
      isLoading: isLoading ?? this.isLoading,
    );
  }
}

class ThemeCubit extends Cubit<ThemeState> {
  final GetThemeSettings _getThemeSettings;
  final ToggleTheme _toggleTheme;
  final UpdateAccentColor _updateAccentColor;

  ThemeCubit({
    required GetThemeSettings getThemeSettings,
    required ToggleTheme toggleTheme,
    required UpdateAccentColor updateAccentColor,
  }) : _getThemeSettings = getThemeSettings,
       _toggleTheme = toggleTheme,
       _updateAccentColor = updateAccentColor,
       super(
         const ThemeState(settings: ThemeSettings.initial(), isLoading: true),
       );

  Future<void> loadTheme() async {
    if (isClosed) return;
    emit(state.copyWith(isLoading: true));
    final settings = await _getThemeSettings();
    if (!isClosed) {
      emit(ThemeState(settings: settings, isLoading: false));
    }
  }

  Future<void> toggleTheme() async {
    if (isClosed) return;
    emit(state.copyWith(isLoading: true));
    final updated = await _toggleTheme(state.settings);
    if (!isClosed) {
      emit(state.copyWith(settings: updated, isLoading: false));
    }
  }

  Future<void> updateAccentColor(int color) async {
    if (color == state.accentColor) return;
    if (isClosed) return;

    emit(state.copyWith(isLoading: true));
    final updated = await _updateAccentColor(state.settings, color);
    if (!isClosed) {
      emit(state.copyWith(settings: updated, isLoading: false));
    }
  }
}
