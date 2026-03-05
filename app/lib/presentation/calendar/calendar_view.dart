import 'package:easy_localization/easy_localization.dart' hide TextDirection;
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/presentation/calendar/bloc/calendar_bloc.dart';
import 'package:focus_flow_app/presentation/calendar/bloc/calendar_event.dart';
import 'package:focus_flow_app/presentation/calendar/bloc/calendar_state.dart';
import 'package:focus_flow_app/presentation/calendar/views/daily_calendar_view.dart';
import 'package:focus_flow_app/presentation/calendar/views/monthly_calendar_view.dart';
import 'package:focus_flow_app/presentation/calendar/views/weekly_calendar_view.dart';
import 'package:focus_flow_app/presentation/calendar/widgets/scheduled_task_form_dialog.dart';

class CalendarView extends StatelessWidget {
  const CalendarView({super.key});

  String _headerTitle(CalendarState state) {
    final d = state.focusedDate;
    switch (state.viewMode) {
      case CalendarViewMode.daily:
        return DateFormat('d MMMM yyyy').format(d);
      case CalendarViewMode.weekly:
        final monday = d.subtract(Duration(days: d.weekday - 1));
        final sunday = monday.add(const Duration(days: 6));
        if (monday.month == sunday.month) {
          return '${monday.day} – ${sunday.day} ${DateFormat('MMMM yyyy').format(monday)}';
        }
        return '${DateFormat('d MMM').format(monday)} – ${DateFormat('d MMM yyyy').format(sunday)}';
      case CalendarViewMode.monthly:
        return DateFormat('MMMM yyyy').format(d);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(context.tr('calendar.title')),
        centerTitle: false,
        actions: [
          BlocBuilder<CalendarBloc, CalendarState>(
            buildWhen: (p, c) => p.isLoading != c.isLoading,
            builder: (_, state) => state.isLoading
                ? const Padding(
                    padding: EdgeInsets.only(right: 16),
                    child: SizedBox(
                      width: 20,
                      height: 20,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    ),
                  )
                : const SizedBox.shrink(),
          ),
        ],
      ),
      floatingActionButton: BlocBuilder<CalendarBloc, CalendarState>(
        builder: (context, state) => FloatingActionButton(
          onPressed: () {
            showDialog(
              context: context,
              builder: (_) => ScheduledTaskFormDialog(
                initialDate: state.focusedDate,
                categories: state.categories,
                onSubmit: ({
                  required String name,
                  String? description,
                  String? categoryId,
                  required int scheduledDate,
                  int? scheduledEndDate,
                }) {
                  context.read<CalendarBloc>().add(
                    CreateScheduledTask(
                      name: name,
                      description: description,
                      categoryId: categoryId,
                      scheduledDate: scheduledDate,
                      scheduledEndDate: scheduledEndDate,
                    ),
                  );
                },
              ),
            );
          },
          child: const Icon(Icons.add),
        ),
      ),
      body: BlocConsumer<CalendarBloc, CalendarState>(
        listener: (context, state) {
          if (state.error != null) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text(state.error!),
                behavior: SnackBarBehavior.floating,
              ),
            );
          }
        },
        builder: (context, state) {
          return Column(
            children: [
              // View mode switcher + navigation
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                child: Row(
                  children: [
                    // Prev
                    IconButton(
                      icon: const Icon(Icons.chevron_left),
                      onPressed: () =>
                          context.read<CalendarBloc>().add(NavigatePrevious()),
                    ),

                    // Header title
                    Expanded(
                      child: GestureDetector(
                        onTap: () => context
                            .read<CalendarBloc>()
                            .add(NavigateToToday()),
                        child: Text(
                          _headerTitle(state),
                          textAlign: TextAlign.center,
                          style:
                              Theme.of(context).textTheme.titleMedium?.copyWith(
                                    fontWeight: FontWeight.w600,
                                  ),
                        ),
                      ),
                    ),

                    // Next
                    IconButton(
                      icon: const Icon(Icons.chevron_right),
                      onPressed: () =>
                          context.read<CalendarBloc>().add(NavigateNext()),
                    ),
                  ],
                ),
              ),

              // View mode tabs
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 12),
                child: SegmentedButton<CalendarViewMode>(
                  segments: [
                    ButtonSegment(
                      value: CalendarViewMode.daily,
                      label: Text(context.tr('calendar.view_daily')),
                      icon: const Icon(Icons.view_day_outlined, size: 16),
                    ),
                    ButtonSegment(
                      value: CalendarViewMode.weekly,
                      label: Text(context.tr('calendar.view_weekly')),
                      icon: const Icon(Icons.view_week_outlined, size: 16),
                    ),
                    ButtonSegment(
                      value: CalendarViewMode.monthly,
                      label: Text(context.tr('calendar.view_monthly')),
                      icon: const Icon(Icons.calendar_month_outlined, size: 16),
                    ),
                  ],
                  selected: {state.viewMode},
                  onSelectionChanged: (modes) {
                    context
                        .read<CalendarBloc>()
                        .add(ChangeViewMode(modes.first));
                  },
                ),
              ),

              const SizedBox(height: 8),
              const Divider(height: 1),
              const SizedBox(height: 4),

              // Calendar content
              Expanded(
                child: Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 8),
                  child: _buildView(state),
                ),
              ),
            ],
          );
        },
      ),
    );
  }

  Widget _buildView(CalendarState state) {
    switch (state.viewMode) {
      case CalendarViewMode.daily:
        return const DailyCalendarView();
      case CalendarViewMode.weekly:
        return const WeeklyCalendarView();
      case CalendarViewMode.monthly:
        return const MonthlyCalendarView();
    }
  }
}
