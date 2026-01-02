import 'package:shared_preferences/shared_preferences.dart';

class ConfigurationService {
  static const String _keyApiBaseUrl = 'api_base_url';
  
  final SharedPreferences _prefs;

  ConfigurationService(this._prefs);

  static Future<ConfigurationService> init() async {
    final prefs = await SharedPreferences.getInstance();
    return ConfigurationService(prefs);
  }

  String? get apiBaseUrl => _prefs.getString(_keyApiBaseUrl);

  Future<void> setApiBaseUrl(String url) async {
    // Remove trailing slash if present for consistency
    final cleanUrl = url.endsWith('/') ? url.substring(0, url.length - 1) : url;
    await _prefs.setString(_keyApiBaseUrl, cleanUrl);
  }

  Future<void> clear() async {
    await _prefs.remove(_keyApiBaseUrl);
  }
}
