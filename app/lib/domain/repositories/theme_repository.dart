import '../entities/theme_settings.dart';

abstract class ThemeRepository {
  Future<ThemeSettings> getThemeSettings();
  Future<ThemeSettings> saveThemeSettings(ThemeSettings settings);
}
