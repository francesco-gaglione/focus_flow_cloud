import 'dart:ui';

import '../entities/user_setting.dart';
import '../repositories/user_settings_repository.dart';

class GetSavedLocale {
  final UserSettingsRepository _repository;

  GetSavedLocale(this._repository);

  Future<Locale?> call() async {
    try {
      final settings = await _repository.getUserSettings();
      final localeSetting = settings.firstWhere(
        (s) => s.key == 'locale',
        orElse: () => UserSetting(key: 'locale', value: ''),
      );

      if (localeSetting.value.isNotEmpty) {
        return Locale(localeSetting.value);
      }
      return null;
    } catch (e) {
      return null;
    }
  }
}
