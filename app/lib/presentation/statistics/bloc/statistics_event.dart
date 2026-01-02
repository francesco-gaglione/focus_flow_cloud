import 'package:equatable/equatable.dart';

abstract class StatisticsEvent extends Equatable {
  const StatisticsEvent();

  @override
  List<Object?> get props => [];
}

class LoadStatistics extends StatisticsEvent {
  final int? startDate;
  final int? endDate;

  const LoadStatistics({this.startDate, this.endDate});

  @override
  List<Object?> get props => [startDate, endDate];
}

class ChangeTimeRange extends StatisticsEvent {
  final StatisticsTimeRange timeRange;

  const ChangeTimeRange(this.timeRange);

  @override
  List<Object?> get props => [timeRange];
}

enum StatisticsTimeRange {
  day,
  yesterday,
  week,
  lastWeek,
  last7Days,
  month,
  last30Days,
}
