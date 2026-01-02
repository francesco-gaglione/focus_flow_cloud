import 'dart:ui';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/domain/repositories/statistics_repository.dart';
import 'package:focus_flow_app/domain/usecases/categories_usecases/get_categories_and_tasks.dart';
import 'package:focus_flow_app/presentation/statistics/bloc/statistics_event.dart';
import 'package:focus_flow_app/presentation/statistics/bloc/statistics_state.dart';
import 'package:logger/logger.dart';

class StatisticsBloc extends Bloc<StatisticsEvent, StatisticsState> {
  final StatisticsRepository _statisticsRepository;
  final GetCategoriesAndTasks _getCategoriesAndTasks;
  final Logger _logger = Logger();

  StatisticsBloc({
    required StatisticsRepository statisticsRepository,
    required GetCategoriesAndTasks getCategoriesAndTasks,
  }) : _statisticsRepository = statisticsRepository,
       _getCategoriesAndTasks = getCategoriesAndTasks,
       super(StatisticsInitial()) {
    on<ChangeTimeRange>(_onChangeTimeRange);
  }

  Future<void> _onChangeTimeRange(
    ChangeTimeRange event,
    Emitter<StatisticsState> emit,
  ) async {
    emit(StatisticsLoading());
    try {
      final now = DateTime.now();
      int startDate;
      int? endDate;

      switch (event.timeRange) {
        case StatisticsTimeRange.day:
          final startOfDay = DateTime(now.year, now.month, now.day);
          startDate = startOfDay.millisecondsSinceEpoch ~/ 1000;
          final endOfDay = startOfDay.add(const Duration(days: 1));
          endDate = endOfDay.millisecondsSinceEpoch ~/ 1000;
          break;
        case StatisticsTimeRange.yesterday:
          final startOfYesterday = DateTime(now.year, now.month, now.day - 1);
          startDate = startOfYesterday.millisecondsSinceEpoch ~/ 1000;
          final endOfYesterday = DateTime(now.year, now.month, now.day);
          endDate = endOfYesterday.millisecondsSinceEpoch ~/ 1000;
          break;
        case StatisticsTimeRange.week:
          // Start of current week (Monday)
          final startOfWeek = DateTime(now.year, now.month, now.day - (now.weekday - 1));
          startDate = startOfWeek.millisecondsSinceEpoch ~/ 1000;
          // End of current week (Next Monday)
          final endOfWeek = startOfWeek.add(const Duration(days: 7));
          endDate = endOfWeek.millisecondsSinceEpoch ~/ 1000;
          break;
        case StatisticsTimeRange.lastWeek:
          // Start of last week (Monday)
          final startOfLastWeek = DateTime(now.year, now.month, now.day - (now.weekday - 1) - 7);
          startDate = startOfLastWeek.millisecondsSinceEpoch ~/ 1000;
          // End of last week (Sunday end / Next Monday start)
          final endOfLastWeek = startOfLastWeek.add(const Duration(days: 7));
          endDate = endOfLastWeek.millisecondsSinceEpoch ~/ 1000;
          break;
        case StatisticsTimeRange.last7Days:
          // Last 7 days including today
          final startOf7Days = DateTime(now.year, now.month, now.day - 6);
          startDate = startOf7Days.millisecondsSinceEpoch ~/ 1000;
          final endOf7Days = DateTime(now.year, now.month, now.day + 1);
          endDate = endOf7Days.millisecondsSinceEpoch ~/ 1000;
          break;
        case StatisticsTimeRange.month:
          final startOfMonth = DateTime(now.year, now.month, 1);
          startDate = startOfMonth.millisecondsSinceEpoch ~/ 1000;
          final nextMonth = DateTime(now.year, now.month + 1, 1);
          endDate = nextMonth.millisecondsSinceEpoch ~/ 1000;
          break;
        case StatisticsTimeRange.last30Days:
          // Last 30 days including today
          final startOf30Days = DateTime(now.year, now.month, now.day - 29);
          startDate = startOf30Days.millisecondsSinceEpoch ~/ 1000;
          final endOf30Days = DateTime(now.year, now.month, now.day + 1);
          endDate = endOf30Days.millisecondsSinceEpoch ~/ 1000;
          break;
      }

      final stats = await _statisticsRepository.calculateStatsByPeriod(
        startDate: startDate,
        endDate: endDate,
      );

      final categoriesResult = await _getCategoriesAndTasks.execute();
      final categoryColors = <String, Color>{};

      if (categoriesResult.success &&
          categoriesResult.categoriesWithTasks != null) {
        for (final item in categoriesResult.categoriesWithTasks!) {
          final category = item.category;
          try {
            categoryColors[category.id] = Color(
              int.parse(category.color.replaceFirst('#', '0xFF')),
            );
          } catch (e) {
            // Fallback or ignore invalid color
            _logger.w(
              'Invalid color for category ${category.name}: ${category.color}',
            );
          }
        }
      }

      emit(
        StatisticsLoaded(
          statistics: stats,
          timeRange: event.timeRange,
          categoryColors: categoryColors,
        ),
      );
    } catch (e) {
      _logger.e('Error changing time range', error: e);
      emit(StatisticsError(e.toString()));
    }
  }
}
