import 'package:package_info_plus/package_info_plus.dart';

class GetAppVersion {
  Future<String> call() async {
    final packageInfo = await PackageInfo.fromPlatform();
    return '${packageInfo.version} (${packageInfo.buildNumber})';
  }
}
