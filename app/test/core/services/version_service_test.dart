import 'package:dio/dio.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:focus_flow_app/core/services/version_service.dart';
import 'package:package_info_plus/package_info_plus.dart';

// Mock Dio using Mocktail or Mockall?
// Project doesn't seem to have mocktail listed in dev_dependencies in step 344?
// It has `mockall` in backend dev-deps (Step 286).
// In app pubspec (Step 344), dev_dependencies: flutter_test. No mockito or mocktail.
// I can implement a FakeDio or just use `flutter_test` capabilities?
// Or better, just add `mocktail`? `flutter pub add --dev mocktail`.
// I'll stick to a simple manual mock for Dio since I can't easily add deps without potentially asking user (though I can).
// I'll create a FakeDio.

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  Dio createMockDio(Map<String, dynamic> responseData) {
    final dio = Dio();
    dio.interceptors.add(InterceptorsWrapper(
      onRequest: (options, handler) {
        if (options.path == '/api/version') {
          handler.resolve(Response(
            requestOptions: options,
            data: responseData,
            statusCode: 200,
          ));
        } else {
          handler.next(options);
        }
      },
      onError: (e, handler) {
        // Prevent actual network calls from crashing if they slip through
        handler.reject(e);
      }
    ));
    return dio;
  }

  group('VersionService', () {
    test('returns true when versions match', () async {
      PackageInfo.setMockInitialValues(
        appName: 'Focus Flow',
        packageName: 'com.example.focus_flow',
        version: '1.0.0',
        buildNumber: '1',
        buildSignature: '',
      );

      final dio = createMockDio({'version': '1.0.0'});
      final service = VersionService(dio);

      expect(await service.checkCompatibility(), true);
    });

    test('returns true when patch differs (Compatible)', () async {
      PackageInfo.setMockInitialValues(
        appName: 'Focus Flow',
        packageName: 'com.example.focus_flow',
        version: '1.0.0',
        buildNumber: '1',
        buildSignature: '',
      );

      final dio = createMockDio({'version': '1.0.1'});
      final service = VersionService(dio);

      expect(await service.checkCompatibility(), true);
    });

    test('returns false when minor differs (Incompatible)', () async {
      PackageInfo.setMockInitialValues(
        appName: 'Focus Flow',
        packageName: 'com.example.focus_flow',
        version: '1.0.0',
        buildNumber: '1',
        buildSignature: '',
      );

      final dio = createMockDio({'version': '1.1.0'});
      final service = VersionService(dio);

      expect(await service.checkCompatibility(), false);
    });

    test('returns false when major differs (Incompatible)', () async {
      PackageInfo.setMockInitialValues(
        appName: 'Focus Flow',
        packageName: 'com.example.focus_flow',
        version: '1.0.0',
        buildNumber: '1',
        buildSignature: '',
      );

      final dio = createMockDio({'version': '2.0.0'});
      final service = VersionService(dio);

      expect(await service.checkCompatibility(), false);
    });
  });
}


