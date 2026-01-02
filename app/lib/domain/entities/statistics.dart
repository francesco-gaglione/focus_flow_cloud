enum ConcentrationPeriod {
  morning('MORNING'),
  afternoon('AFTERNOON');

  final String value;
  const ConcentrationPeriod(this.value);

  static ConcentrationPeriod fromString(String value) {
    switch (value) {
      case 'MORNING':
        return ConcentrationPeriod.morning;
      case 'AFTERNOON':
        return ConcentrationPeriod.afternoon;
      default:
        throw ArgumentError('Invalid ConcentrationPeriod: $value');
    }
  }
}

class CategoryDistribution {
  final String categoryId;
  final String categoryName;
  final int totalFocusTime;
  final double percentage;

  CategoryDistribution({
    required this.categoryId,
    required this.categoryName,
    required this.totalFocusTime,
    required this.percentage,
  });
}

class TaskDistribution {
  final String taskName;
  final String? categoryId;
  final String? categoryName;
  final int totalFocusTime;
  final double percentage;

  TaskDistribution({
    required this.taskName,
    this.categoryId,
    this.categoryName,
    required this.totalFocusTime,
    required this.percentage,
  });
}

class DailyActivityDistribution {
  final String categoryId;
  final String categoryName;
  final int totalFocusTime;

  DailyActivityDistribution({
    required this.categoryId,
    required this.categoryName,
    required this.totalFocusTime,
  });
}

class DailyActivity {
  final int date;
  final List<DailyActivityDistribution> categoryDistribution;

  DailyActivity({required this.date, required this.categoryDistribution});
}

class PeriodStatistics {
  final int totalSessions;
  final int totalBreaks;
  final int totalFocusTime;
  final int totalBreakTime;
  final ConcentrationPeriod mostConcentratedPeriod;
  final ConcentrationPeriod lessConcentratedPeriod;
  final List<int> concentrationDistribution;
  final List<CategoryDistribution> categoryDistribution;
  final List<TaskDistribution> taskDistribution;
  final List<DailyActivity> dailyActivity;

  PeriodStatistics({
    required this.totalSessions,
    required this.totalBreaks,
    required this.totalFocusTime,
    required this.totalBreakTime,
    required this.mostConcentratedPeriod,
    required this.lessConcentratedPeriod,
    required this.concentrationDistribution,
    required this.categoryDistribution,
    required this.taskDistribution,
    required this.dailyActivity,
  });
}
