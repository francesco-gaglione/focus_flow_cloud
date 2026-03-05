import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/core/di/service_locator.dart';
import 'package:focus_flow_app/presentation/calendar/bloc/calendar_bloc.dart';
import 'package:focus_flow_app/presentation/calendar/bloc/calendar_event.dart';
import 'package:focus_flow_app/presentation/calendar/calendar_view.dart';

class CalendarPage extends StatelessWidget {
  const CalendarPage({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (_) => sl<CalendarBloc>()..add(LoadCalendarData()),
      child: const CalendarView(),
    );
  }
}
