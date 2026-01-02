import 'dart:async';

import 'package:easy_localization/easy_localization.dart' hide TextDirection;
import 'package:flutter/material.dart';
import 'dart:math' as math;

import 'package:logger/logger.dart';

import 'package:focus_flow_app/adapters/dtos/ws_dtos.dart';

class FocusTimerWidget extends StatefulWidget {
  final DateTime? startDate;
  final VoidCallback onStart;
  final VoidCallback onBreak;
  final VoidCallback onTerminate;
  final SessionTypeEnum? sessionType;

  const FocusTimerWidget({
    super.key,
    this.startDate,
    required this.onStart,
    required this.onBreak,
    required this.onTerminate,
    this.sessionType,
  });

  @override
  State<FocusTimerWidget> createState() => _FocusTimerWidgetState();
}

class _FocusTimerWidgetState extends State<FocusTimerWidget>
    with SingleTickerProviderStateMixin {
  final Logger logger = Logger();
  late AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 4),
    )..repeat();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  int _getTotalSeconds(SessionTypeEnum? sessionType) {
    switch (sessionType) {
      case SessionTypeEnum.focus:
      case SessionTypeEnum.work:
        return 25 * 60;
      case SessionTypeEnum.shortBreak:
        return 5 * 60;
      case SessionTypeEnum.longBreak:
        return 15 * 60;
      default:
        return 25 * 60;
    }
  }

  String _getTitle(BuildContext context, SessionTypeEnum? sessionType) {
    switch (sessionType) {
      case SessionTypeEnum.focus:
      case SessionTypeEnum.work:
        return context.tr('focus.session_title');
      case SessionTypeEnum.shortBreak:
        return context.tr('focus.short_break_title');
      case SessionTypeEnum.longBreak:
        return context.tr('focus.long_break_title');
      default:
        return context.tr('focus.session_title');
    }
  }

  String _getStatusText(BuildContext context, SessionTypeEnum? sessionType) {
    int elapsed = 0;
    if (widget.startDate != null) {
      final now = DateTime.now();
      elapsed = now.difference(widget.startDate!).inSeconds;
    }

    if (widget.startDate != null && elapsed > _getTotalSeconds(sessionType)) {
      return context.tr('focus.overtime');
    }

    switch (sessionType) {
      case SessionTypeEnum.focus:
      case SessionTypeEnum.work:
        return context.tr('focus.focusing');

      case SessionTypeEnum.shortBreak:
        return context.tr('focus.short_break_status');

      case SessionTypeEnum.longBreak:
        return context.tr('focus.long_break_status');

      default:
        return context.tr('focus.focusing');
    }
  }

  @override
  Widget build(BuildContext context) {
    // logger.i('Building FocusTimerWidget, startDate: ${widget.startDate}');

    final bool isRunning = widget.startDate != null;
    final colorScheme = Theme.of(context).colorScheme;

    // Use theme colors instead of hardcoded values
    final accentColor = colorScheme.primary;
    final isBreak = widget.sessionType == SessionTypeEnum.shortBreak ||
        widget.sessionType == SessionTypeEnum.longBreak;
    // Use gray for breaks or when not running (stopped), otherwise use accent color
    final indicatorColor =
        (isBreak || !isRunning) ? Colors.grey : accentColor;
    final trackColor = colorScheme.surfaceContainerHighest.withAlpha(
      (255 * 0.3).round(),
    );
    final textColor = colorScheme.onSurface;
    final errorColor = colorScheme.error;

    return StreamBuilder<int>(
      stream:
          isRunning
              ? Stream.periodic(const Duration(seconds: 1), (i) => i)
              : null,
      builder: (context, snapshot) {
        final totalSeconds = _getTotalSeconds(widget.sessionType);
        int remainingSeconds;
        int overtimeSeconds = 0;
        bool isOvertime = false;

        if (isRunning) {
          final now = DateTime.now();
          final elapsed = now.difference(widget.startDate!).inSeconds;
          if (elapsed > totalSeconds) {
            remainingSeconds = 0;
            overtimeSeconds = elapsed - totalSeconds;
            isOvertime = true;
          } else {
            remainingSeconds = totalSeconds - elapsed;
          }
        } else {
          remainingSeconds = totalSeconds;
        }

        final progress =
            totalSeconds > 0 ? remainingSeconds / totalSeconds : 0.0;

        String formatTime(int seconds) {
          final minutes = seconds ~/ 60;
          final secs = seconds % 60;
          return '${minutes.toString().padLeft(2, '0')}:${secs.toString().padLeft(2, '0')}';
        }

        final displayTime =
            isOvertime
                ? formatTime(totalSeconds + overtimeSeconds)
                : formatTime(remainingSeconds);

        final timerColor = isOvertime ? errorColor : textColor;

        return Container(
          decoration: BoxDecoration(
            color: colorScheme.surfaceContainerHighest.withAlpha(
              (255 * 0.3).round(),
            ),
            borderRadius: BorderRadius.circular(32),
            border: Border.all(
              color: colorScheme.outlineVariant.withAlpha((255 * 0.2).round()),
            ),
            boxShadow: [
              BoxShadow(
                color: Colors.black.withAlpha((255 * 0.05).round()),
                blurRadius: 20,
                offset: const Offset(0, 10),
              ),
            ],
          ),
          child: Padding(
            padding: const EdgeInsets.all(32),
            child: Column(
              children: [
                Text(
                  _getTitle(context, widget.sessionType),
                  style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                    fontWeight: FontWeight.bold,
                    color: textColor,
                    letterSpacing: 0.5,
                  ),
                ),
                const SizedBox(height: 40),
                SizedBox(
                  width: 300,
                  height: 300,
                  child: Stack(
                    alignment: Alignment.center,
                    children: [
                      // Water Wave Timer
                      ClipOval(
                        child: Container(
                          width: 300,
                          height: 300,
                          color: trackColor, // Background color for empty part
                          child: AnimatedBuilder(
                            animation: _controller,
                            builder: (context, child) {
                              return CustomPaint(
                                painter: _WaterWavePainter(
                                  progress: isOvertime ? 0.0 : progress,
                                  color: isOvertime ? errorColor : indicatorColor,
                                  animationValue: _controller.value,
                                ),
                              );
                            },
                          ),
                        ),
                      ),
                      // Border Ring
                      Container(
                        width: 300,
                        height: 300,
                        decoration: BoxDecoration(
                          shape: BoxShape.circle,
                          border: Border.all(
                            color: (isOvertime ? errorColor : indicatorColor)
                                .withValues(alpha: 0.3),
                            width: 4,
                          ),
                        ),
                      ),
                      Column(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: [
                          Text(
                            displayTime,
                            style: Theme.of(
                              context,
                            ).textTheme.displayLarge?.copyWith(
                              fontWeight: FontWeight.w900,
                              fontSize: 72,
                              color: timerColor,
                              letterSpacing: -2.0,
                              height: 1.0,
                              shadows: [
                                Shadow(
                                  color: Colors.black.withValues(alpha: 0.3),
                                  blurRadius: 10,
                                  offset: const Offset(0, 2),
                                ),
                              ],
                            ),
                          ),
                          const SizedBox(height: 12),
                          Container(
                            padding: const EdgeInsets.symmetric(
                              horizontal: 16,
                              vertical: 8,
                            ),
                            decoration: BoxDecoration(
                              color: (isOvertime ? errorColor : indicatorColor)
                                  .withAlpha((255 * 0.1).round()),
                              borderRadius: BorderRadius.circular(20),
                            ),
                            child: Text(
                              _getStatusText(context, widget.sessionType),
                              style: Theme.of(
                                context,
                              ).textTheme.titleMedium?.copyWith(
                                color: isOvertime ? errorColor : indicatorColor,
                                fontWeight: FontWeight.bold,
                                shadows: [
                                  Shadow(
                                    color: Colors.black.withValues(alpha: 0.2),
                                    blurRadius: 4,
                                    offset: const Offset(0, 1),
                                  ),
                                ],
                              ),
                            ),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 48),

                if (!isRunning)
                  SizedBox(
                    width: double.infinity,
                    height: 64,
                    child: FilledButton(
                      onPressed: widget.onStart,
                      style: FilledButton.styleFrom(
                        backgroundColor: accentColor,
                        foregroundColor: colorScheme.onPrimary,
                        elevation: 8,
                        shadowColor: accentColor.withAlpha((255 * 0.4).round()),
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(24),
                        ),
                        textStyle: const TextStyle(
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                          letterSpacing: 1.0,
                        ),
                      ),
                      child: Text(context.tr('focus.start').toUpperCase()),
                    ),
                  )
                else if (widget.sessionType == SessionTypeEnum.shortBreak ||
                    widget.sessionType == SessionTypeEnum.longBreak)
                  Row(
                    children: [
                      Expanded(
                        child: SizedBox(
                          height: 64,
                          child: FilledButton(
                            onPressed: widget.onStart,
                            style: FilledButton.styleFrom(
                              backgroundColor: accentColor,
                              foregroundColor: colorScheme.onPrimary,
                              elevation: 8,
                              shadowColor: accentColor.withAlpha(
                                (255 * 0.4).round(),
                              ),
                              shape: RoundedRectangleBorder(
                                borderRadius: BorderRadius.circular(24),
                              ),
                              textStyle: const TextStyle(
                                fontSize: 18,
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                            child: Text(
                              context.tr('focus.start').toUpperCase(),
                            ),
                          ),
                        ),
                      ),
                      const SizedBox(width: 16),
                      Expanded(
                        child: SizedBox(
                          height: 64,
                          child: OutlinedButton(
                            onPressed: widget.onTerminate,
                            style: OutlinedButton.styleFrom(
                              foregroundColor: textColor.withAlpha(
                                (255 * 0.8).round(),
                              ),
                              side: BorderSide(
                                color: colorScheme.outline.withAlpha(
                                  (255 * 0.3).round(),
                                ),
                                width: 2,
                              ),
                              shape: RoundedRectangleBorder(
                                borderRadius: BorderRadius.circular(24),
                              ),
                              textStyle: const TextStyle(
                                fontSize: 18,
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                            child: Text(context.tr('focus.terminate')),
                          ),
                        ),
                      ),
                    ],
                  )
                else
                  Row(
                    children: [
                      Expanded(
                        child: SizedBox(
                          height: 64,
                          child: FilledButton(
                            onPressed: widget.onBreak,
                            style: FilledButton.styleFrom(
                              backgroundColor: accentColor,
                              foregroundColor: colorScheme.onPrimary,
                              elevation: 8,
                              shadowColor: accentColor.withAlpha(
                                (255 * 0.4).round(),
                              ),
                              shape: RoundedRectangleBorder(
                                borderRadius: BorderRadius.circular(24),
                              ),
                              textStyle: const TextStyle(
                                fontSize: 18,
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                            child: Text(
                              context.tr('focus.break').toUpperCase(),
                            ),
                          ),
                        ),
                      ),
                      const SizedBox(width: 16),
                      Expanded(
                        child: SizedBox(
                          height: 64,
                          child: OutlinedButton(
                            onPressed: widget.onTerminate,
                            style: OutlinedButton.styleFrom(
                              foregroundColor: textColor.withAlpha(
                                (255 * 0.8).round(),
                              ),
                              side: BorderSide(
                                color: colorScheme.outline.withAlpha(
                                  (255 * 0.3).round(),
                                ),
                                width: 2,
                              ),
                              shape: RoundedRectangleBorder(
                                borderRadius: BorderRadius.circular(24),
                              ),
                              textStyle: const TextStyle(
                                fontSize: 18,
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                            child: Text(context.tr('focus.terminate')),
                          ),
                        ),
                      ),
                    ],
                  ),
              ],
            ),
          ),
        );
      },
    );
  }
}

/// Custom painter for water wave effect
class _WaterWavePainter extends CustomPainter {
  final double progress; // 0.0 to 1.0 (1.0 = full)
  final Color color;
  final double animationValue;

  _WaterWavePainter({
    required this.progress,
    required this.color,
    required this.animationValue,
  });

  @override
  void paint(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = color.withValues(alpha: 0.6)
      ..style = PaintingStyle.fill;

    final path = Path();
    
    // Calculate water level height from bottom
    final waterHeight = size.height * progress;
    final baseHeight = size.height - waterHeight;

    path.moveTo(0, baseHeight);

    // Draw wave
    // Wave parameters
    const waveAmplitude = 10.0;
    const waveFrequency = 2.0;
    
    for (double x = 0; x <= size.width; x++) {
      final y = baseHeight + 
          math.sin((x / size.width * waveFrequency * math.pi * 2) + (animationValue * math.pi * 2)) * waveAmplitude;
      path.lineTo(x, y);
    }

    path.lineTo(size.width, size.height);
    path.lineTo(0, size.height);
    path.close();

    canvas.drawPath(path, paint);

    // Draw a second wave behind with slightly different phase/opacity for depth
    final paint2 = Paint()
      ..color = color.withValues(alpha: 0.4)
      ..style = PaintingStyle.fill;

    final path2 = Path();
    path2.moveTo(0, baseHeight);

    for (double x = 0; x <= size.width; x++) {
      final y = baseHeight + 
          math.sin((x / size.width * waveFrequency * math.pi * 2) + (animationValue * math.pi * 2) + math.pi) * waveAmplitude;
      path2.lineTo(x, y);
    }

    path2.lineTo(size.width, size.height);
    path2.lineTo(0, size.height);
    path2.close();

    canvas.drawPath(path2, paint2);
  }

  @override
  bool shouldRepaint(_WaterWavePainter oldDelegate) {
    return oldDelegate.progress != progress ||
        oldDelegate.color != color ||
        oldDelegate.animationValue != animationValue;
  }
}
