import '../entities/theme_settings.dart';
import '../repositories/theme_repository.dart';

class UpdateAccentColor {
  final ThemeRepository _repository;

  UpdateAccentColor(this._repository);

  Future<ThemeSettings> call(ThemeSettings current, int accentColor) async {
    final updated = current.copyWith(accentColor: accentColor);
    return _repository.saveThemeSettings(updated);
  }
}

