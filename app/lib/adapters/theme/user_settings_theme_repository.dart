import 'package:focus_flow_app/domain/entities/theme_settings.dart';
import 'package:focus_flow_app/domain/entities/user_setting.dart';
import 'package:focus_flow_app/domain/repositories/theme_repository.dart';
import 'package:focus_flow_app/domain/repositories/user_settings_repository.dart';

class UserSettingsThemeRepository implements ThemeRepository {
  final UserSettingsRepository _userSettingsRepository;
  
  static const String _keyThemeMode = 'theme_mode';
  static const String _keyAccentColor = 'accent_color';

  UserSettingsThemeRepository(this._userSettingsRepository);

  @override
  Future<ThemeSettings> getThemeSettings() async {
    try {
      final settings = await _userSettingsRepository.getUserSettings();
      
      bool isDarkMode = false; // Default to light
      int accentColor = 0xFF2196F3; // Default blue
      
      for (final setting in settings) {
        if (setting.key == _keyThemeMode) {
          isDarkMode = setting.value == 'dark';
        } else if (setting.key == _keyAccentColor) {
          accentColor = int.tryParse(setting.value) ?? accentColor;
        }
      }
      
      return ThemeSettings(
        isDarkMode: isDarkMode,
        accentColor: accentColor,
      );
    } catch (e) {
      // Fallback to default if fetch fails
      return const ThemeSettings.initial();
    }
  }

  @override
  Future<ThemeSettings> saveThemeSettings(ThemeSettings settings) async {
    try {
      await _userSettingsRepository.updateUserSetting(
        UserSetting(
          key: _keyThemeMode,
          value: settings.isDarkMode ? 'dark' : 'light',
        ),
      );
      
      await _userSettingsRepository.updateUserSetting(
        UserSetting(
          key: _keyAccentColor,
          value: settings.accentColor.toString(),
        ),
      );
      
      return settings;
    } catch (e) {
      // If save fails, return the settings anyway so the UI updates optimistically
      // or rethrow if we want to show an error
      return settings;
    }
  }
}
