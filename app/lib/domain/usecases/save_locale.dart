import 'dart:ui';

import '../entities/user_setting.dart';
import '../repositories/user_settings_repository.dart';

class SaveLocale {
  final UserSettingsRepository _repository;

  SaveLocale(this._repository);

  Future<void> call(Locale locale) async {
    await _repository.updateUserSetting(
      UserSetting(key: 'locale', value: locale.languageCode),
    );
  }
}
