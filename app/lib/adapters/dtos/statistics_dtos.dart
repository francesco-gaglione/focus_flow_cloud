import 'package:freezed_annotation/freezed_annotation.dart';
import '../../domain/entities/statistics.dart';

part 'statistics_dtos.freezed.dart';
part 'statistics_dtos.g.dart';

// Request DTOs

@freezed
class GetStatsByPeriodDto with _$GetStatsByPeriodDto {
  const factory GetStatsByPeriodDto({required int startDate, int? endDate}) =
      _GetStatsByPeriodDto;

  factory GetStatsByPeriodDto.fromJson(Map<String, dynamic> json) =>
      _$GetStatsByPeriodDtoFromJson(json);
}

// Response DTOs

@freezed
class CategoryDistributionDto with _$CategoryDistributionDto {
  const factory CategoryDistributionDto({
    required String categoryId,
    required String categoryName,
    required int totalFocusTime,
    required double percentage,
    required List<TaskDistributionDto> taskDistribution,
  }) = _CategoryDistributionDto;

  factory CategoryDistributionDto.fromJson(Map<String, dynamic> json) =>
      _$CategoryDistributionDtoFromJson(json);

  factory CategoryDistributionDto.fromEntity(CategoryDistribution entity) {
    return CategoryDistributionDto(
      categoryId: entity.categoryId,
      categoryName: entity.categoryName,
      totalFocusTime: entity.totalFocusTime,
      percentage: entity.percentage,
      taskDistribution:
          entity.taskDistribution
              .map((dist) => TaskDistributionDto.fromEntity(dist))
              .toList(),
    );
  }
}

@freezed
class TaskDistributionDto with _$TaskDistributionDto {
  const factory TaskDistributionDto({
    required String taskName,
    required int totalFocusTime,
    required double percentage,
  }) = _TaskDistributionDto;

  factory TaskDistributionDto.fromJson(Map<String, dynamic> json) =>
      _$TaskDistributionDtoFromJson(json);

  factory TaskDistributionDto.fromEntity(TaskDistribution entity) {
    return TaskDistributionDto(
      taskName: entity.taskName,
      totalFocusTime: entity.totalFocusTime,
      percentage: entity.percentage,
    );
  }
}

@freezed
class DailyActivityDistributionDto with _$DailyActivityDistributionDto {
  const factory DailyActivityDistributionDto({
    required String categoryId,
    required String categoryName,
    required int totalFocusTime,
  }) = _DailyActivityDistributionDto;

  factory DailyActivityDistributionDto.fromJson(Map<String, dynamic> json) =>
      _$DailyActivityDistributionDtoFromJson(json);

  factory DailyActivityDistributionDto.fromEntity(
    DailyActivityDistribution entity,
  ) {
    return DailyActivityDistributionDto(
      categoryId: entity.categoryId,
      categoryName: entity.categoryName,
      totalFocusTime: entity.totalFocusTime,
    );
  }
}

@freezed
class DailyActivityDto with _$DailyActivityDto {
  const factory DailyActivityDto({
    required int date,
    required List<DailyActivityDistributionDto> categoryDistribution,
  }) = _DailyActivityDto;

  factory DailyActivityDto.fromJson(Map<String, dynamic> json) =>
      _$DailyActivityDtoFromJson(json);

  factory DailyActivityDto.fromEntity(DailyActivity entity) {
    return DailyActivityDto(
      date: entity.date,
      categoryDistribution:
          entity.categoryDistribution
              .map((dist) => DailyActivityDistributionDto.fromEntity(dist))
              .toList(),
    );
  }
}

@freezed
class GetStatsByPeriodResponseDto with _$GetStatsByPeriodResponseDto {
  const factory GetStatsByPeriodResponseDto({
    required int totalSessions,
    required int totalBreaks,
    required int totalFocusTime,
    required int totalBreakTime,
    required String mostConcentratedPeriod,
    required String lessConcentratedPeriod,
    required List<int> concentrationDistribution,
    required List<CategoryDistributionDto> categoryDistribution,
    required List<DailyActivityDto> dailyActivity,
  }) = _GetStatsByPeriodResponseDto;

  factory GetStatsByPeriodResponseDto.fromJson(Map<String, dynamic> json) =>
      _$GetStatsByPeriodResponseDtoFromJson(json);

  factory GetStatsByPeriodResponseDto.fromEntity(PeriodStatistics entity) {
    return GetStatsByPeriodResponseDto(
      totalSessions: entity.totalSessions,
      totalBreaks: entity.totalBreaks,
      totalFocusTime: entity.totalFocusTime,
      totalBreakTime: entity.totalBreakTime,
      mostConcentratedPeriod: entity.mostConcentratedPeriod.value,
      lessConcentratedPeriod: entity.lessConcentratedPeriod.value,
      concentrationDistribution: entity.concentrationDistribution,
      categoryDistribution:
          entity.categoryDistribution
              .map((dist) => CategoryDistributionDto.fromEntity(dist))
              .toList(),
      dailyActivity:
          entity.dailyActivity
              .map((activity) => DailyActivityDto.fromEntity(activity))
              .toList(),
    );
  }
}
