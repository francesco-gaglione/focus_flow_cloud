import '../entities/theme_settings.dart';
import '../repositories/theme_repository.dart';

class ToggleTheme {
  final ThemeRepository _repository;

  ToggleTheme(this._repository);

  Future<ThemeSettings> call(ThemeSettings current) async {
    final next = current.copyWith(isDarkMode: !current.isDarkMode);
    return _repository.saveThemeSettings(next);
  }
}
