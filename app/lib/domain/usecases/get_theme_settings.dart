import '../entities/theme_settings.dart';
import '../repositories/theme_repository.dart';

class GetThemeSettings {
  final ThemeRepository _repository;

  GetThemeSettings(this._repository);

  Future<ThemeSettings> call() {
    return _repository.getThemeSettings();
  }
}
