import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/adapters/dtos/ws_dtos.dart';
import 'package:focus_flow_app/presentation/focus/bloc/focus_bloc.dart';
import 'package:focus_flow_app/presentation/focus/bloc/focus_state.dart';

class MinimalistTimerHeader extends StatefulWidget {
  const MinimalistTimerHeader({super.key});

  @override
  State<MinimalistTimerHeader> createState() => _MinimalistTimerHeaderState();
}

class _MinimalistTimerHeaderState extends State<MinimalistTimerHeader> {
  Timer? _timer;
  
  @override
  void initState() {
    super.initState();
    _startTimer();
  }

  @override
  void dispose() {
    _timer?.cancel();
    super.dispose();
  }

  void _startTimer() {
    _timer = Timer.periodic(const Duration(seconds: 1), (_) {
      if (mounted) setState(() {});
    });
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

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<FocusBloc, FocusState>(
      builder: (context, state) {
        final sessionState = state.sessionState;
        final startDate = sessionState != null
            ? DateTime.fromMillisecondsSinceEpoch(sessionState.startDate * 1000)
            : null;
        final sessionType = sessionState?.sessionType;
        
        // Only show if timer is actually running
        final bool isRunning = startDate != null;
        
        if (!isRunning) {
          return const SizedBox.shrink();
        }

        final totalSeconds = _getTotalSeconds(sessionType);
        int remainingSeconds;
        bool isOvertime = false;

        final now = DateTime.now();
        final elapsed = now.difference(startDate).inSeconds;
        
        if (elapsed > totalSeconds) {
          remainingSeconds = elapsed - totalSeconds;
          isOvertime = true;
        } else {
          remainingSeconds = totalSeconds - elapsed;
        }

        String formatTime(int seconds) {
          final minutes = seconds ~/ 60;
          final secs = seconds % 60;
          return '${minutes.toString().padLeft(2, '0')}:${secs.toString().padLeft(2, '0')}';
        }

        final baseTime = formatTime(remainingSeconds);
        final displayTime = isOvertime ? '+$baseTime' : baseTime;
        
        final theme = Theme.of(context);
        final colorScheme = theme.colorScheme;
        
        final isBreak = sessionType == SessionTypeEnum.shortBreak ||
             sessionType == SessionTypeEnum.longBreak;
             
        // Dynamic Island Style Colors
        final backgroundColor = colorScheme.inverseSurface;
        final contentColor = colorScheme.onInverseSurface;
        
        final statusColor = isOvertime 
            ? colorScheme.error 
            : (isBreak ? Colors.orangeAccent : Colors.greenAccent);

        IconData icon;
        String statusText;
        
        if (isBreak) {
          icon = Icons.coffee;
          statusText = 'Break';
        } else {
          icon = Icons.timer;
          statusText = 'Focus';
        }

        return Container(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
          decoration: BoxDecoration(
            color: backgroundColor,
            borderRadius: BorderRadius.circular(32),
            boxShadow: [
              BoxShadow(
                color: Colors.black.withAlpha(50),
                blurRadius: 10,
                offset: const Offset(0, 4),
              ),
            ],
          ),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Icon(icon, size: 18, color: statusColor),
              const SizedBox(width: 8),
              Text(
                statusText,
                style: theme.textTheme.labelMedium?.copyWith(
                  color: contentColor,
                  fontWeight: FontWeight.bold,
                ),
              ),
              const SizedBox(width: 12),
              Container(
                width: 1,
                height: 16,
                color: contentColor.withAlpha(50),
              ),
              const SizedBox(width: 12),
              Text(
                displayTime,
                style: theme.textTheme.titleMedium?.copyWith(
                  color: isOvertime ? statusColor : contentColor,
                  fontWeight: FontWeight.bold,
                  fontFeatures: [const FontFeature.tabularFigures()],
                  letterSpacing: 1.0,
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}
