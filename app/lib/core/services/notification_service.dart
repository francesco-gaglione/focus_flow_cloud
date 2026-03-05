import 'package:flutter/foundation.dart';
import 'package:flutter_local_notifications/flutter_local_notifications.dart';
import 'package:focus_flow_app/adapters/dtos/ws_dtos.dart';
import 'package:logger/logger.dart';

class NotificationService {
  final Logger _logger = Logger();
  final FlutterLocalNotificationsPlugin _plugin =
      FlutterLocalNotificationsPlugin();
  bool _initialized = false;

  static const _channelId = 'focus_flow_timer';
  static const _channelName = 'Focus Timer';

  Future<void> initialize() async {
    // flutter_local_notifications does not support web
    if (kIsWeb) return;

    const androidInit =
        AndroidInitializationSettings('@mipmap/ic_launcher');
    const darwinInit = DarwinInitializationSettings(
      requestAlertPermission: true,
      requestBadgePermission: false,
      requestSoundPermission: true,
    );
    const linuxInit =
        LinuxInitializationSettings(defaultActionName: 'Open Focus Flow');

    const initSettings = InitializationSettings(
      android: androidInit,
      iOS: darwinInit,
      macOS: darwinInit,
      linux: linuxInit,
    );

    await _plugin.initialize(initSettings);
    _initialized = true;
    _logger.i('NotificationService initialized');
  }

  Future<void> showTimerExpiredNotification(
    SessionTypeEnum? sessionType,
  ) async {
    if (!_initialized) return;

    final isBreak =
        sessionType == SessionTypeEnum.shortBreak ||
        sessionType == SessionTypeEnum.longBreak;

    final body =
        isBreak
            ? 'Your break has ended. Ready to focus again?'
            : 'Your focus session has finished. Time for a break!';

    const androidDetails = AndroidNotificationDetails(
      _channelId,
      _channelName,
      importance: Importance.high,
      priority: Priority.high,
      playSound: true,
    );
    const darwinDetails = DarwinNotificationDetails(
      presentAlert: true,
      presentSound: true,
    );
    const linuxDetails = LinuxNotificationDetails(
      urgency: LinuxNotificationUrgency.normal,
    );
    const details = NotificationDetails(
      android: androidDetails,
      iOS: darwinDetails,
      macOS: darwinDetails,
      linux: linuxDetails,
    );

    try {
      await _plugin.show(0, "Time's up!", body, details);
      _logger.i('Timer expired notification sent');
    } catch (e) {
      _logger.e('Failed to show notification', error: e);
    }
  }
}
