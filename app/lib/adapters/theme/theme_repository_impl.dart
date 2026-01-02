import 'package:focus_flow_app/domain/entities/theme_settings.dart';
import 'package:focus_flow_app/domain/repositories/theme_repository.dart';

class InMemoryThemeRepositoryImpl implements ThemeRepository {
  ThemeSettings _current = const ThemeSettings.initial();

  @override
  Future<ThemeSettings> getThemeSettings() async {
    // In a real case, you might read from SharedPreferences/local storage.
    return _current;
  }

  @override
  Future<ThemeSettings> saveThemeSettings(ThemeSettings settings) async {
    _current = settings;
    return _current;
  }
}
