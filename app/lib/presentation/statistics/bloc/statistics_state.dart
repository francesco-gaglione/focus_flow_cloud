import 'dart:ui';
import 'package:equatable/equatable.dart';
import 'package:focus_flow_app/domain/entities/statistics.dart';
import 'package:focus_flow_app/presentation/statistics/bloc/statistics_event.dart';

abstract class StatisticsState extends Equatable {
  const StatisticsState();

  @override
  List<Object?> get props => [];
}

class StatisticsInitial extends StatisticsState {}

class StatisticsLoading extends StatisticsState {}

class StatisticsLoaded extends StatisticsState {
  final PeriodStatistics statistics;
  final StatisticsTimeRange timeRange;
  final Map<String, Color> categoryColors;

  const StatisticsLoaded({
    required this.statistics,
    required this.timeRange,
    required this.categoryColors,
  });

  @override
  List<Object?> get props => [statistics, timeRange, categoryColors];
}

class StatisticsError extends StatisticsState {
  final String message;

  const StatisticsError(this.message);

  @override
  List<Object?> get props => [message];
}
