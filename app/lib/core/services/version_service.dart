import 'package:dio/dio.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:logger/logger.dart';

class VersionService {
  final Dio _dio;
  final Logger _logger = Logger();

  VersionService(this._dio);

  Future<String> getBackendVersion() async {
    try {
      Response response;
      try {
        response = await _dio.get('/api/version');
      } catch (e) {
        response = await _dio.get('/api/version/');
      }

      return response.data['version'];
    } catch (e) {
      _logger.e('Failed to fetch backend version: $e');
      throw Exception('Failed to fetch backend version');
    }
  }

  Future<bool> checkCompatibility() async {
    try {
      final backendVersion = await getBackendVersion();
      final packageInfo = await PackageInfo.fromPlatform();
      final appVersion = packageInfo.version;

      _logger.i('App Version: $appVersion, Backend Version: $backendVersion');

      // Logic: mismatch if Major or Minor differs.
      final backendParts = backendVersion.split('.').map(int.parse).toList();
      final appParts = appVersion.split('.').map(int.parse).toList();
      
      if (backendParts.length < 2 || appParts.length < 2) {
        return true; // Safe fallback
      }

      if (backendParts[0] != appParts[0] || backendParts[1] != appParts[1]) {
        _logger.w(
          'Version Mismatch: App($appVersion) vs Backend($backendVersion)',
        );
        return false; // Incompatible
      }

      return true; // Compatible
    } catch (e) {
      _logger.e('Error checking compatibility: $e');
      // If we can't check, assume compatible or handle error?
      // User wants warning on mismatch. If network fails, probably don't warn about version yet.
      return true;
    }
  }
}
