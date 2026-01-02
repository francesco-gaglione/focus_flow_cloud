import 'package:dio/dio.dart';
import 'package:flutter/foundation.dart';
import 'package:focus_flow_app/domain/entities/user_setting.dart';
import 'package:focus_flow_app/domain/repositories/user_settings_repository.dart';
import 'package:logger/logger.dart';

class HttpUserSettingsRepository implements UserSettingsRepository {
  final Dio _dio;
  final String baseUrl;
  final Logger _logger;

  HttpUserSettingsRepository({
    required Dio dio,
    this.baseUrl = 'http://localhost:3000',
  }) : _dio = dio,
       _logger = Logger(
         printer: SimplePrinter(printTime: true),
         level: kDebugMode ? Level.debug : Level.warning,
       );

  @override
  Future<List<UserSetting>> getUserSettings() async {
    try {
      final response = await _dio.get('$baseUrl/api/setting');

      if (response.statusCode == 200) {
        final data = response.data as Map<String, dynamic>;
        final settingsList = data['settings'] as List;
        return settingsList
            .map(
              (e) => UserSetting(
                key: e['key'] as String,
                value: e['value'] as String,
              ),
            )
            .toList();
      } else {
        _logger.w('Failed to fetch user settings: ${response.statusCode}');
        return [];
      }
    } catch (e) {
      _logger.e('Error fetching user settings', error: e);
      return [];
    }
  }

  @override
  Future<bool> updateUserSetting(UserSetting setting) async {
    try {
      final response = await _dio.patch(
        '$baseUrl/api/setting',
        data: {'key': setting.key, 'value': setting.value},
      );

      if (response.statusCode == 200) {
        return true;
      } else {
        _logger.w('Failed to update user setting: ${response.statusCode}');
        return false;
      }
    } catch (e) {
      _logger.e('Error updating user setting', error: e);
      return false;
    }
  }
}
