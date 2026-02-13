// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'statistics_dtos.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_GetStatsByPeriodDto _$GetStatsByPeriodDtoFromJson(Map<String, dynamic> json) =>
    _GetStatsByPeriodDto(
      startDate: (json['startDate'] as num).toInt(),
      endDate: (json['endDate'] as num?)?.toInt(),
    );

Map<String, dynamic> _$GetStatsByPeriodDtoToJson(
  _GetStatsByPeriodDto instance,
) => <String, dynamic>{
  'startDate': instance.startDate,
  'endDate': instance.endDate,
};

_CategoryDistributionDto _$CategoryDistributionDtoFromJson(
  Map<String, dynamic> json,
) => _CategoryDistributionDto(
  categoryId: json['categoryId'] as String,
  categoryName: json['categoryName'] as String,
  totalFocusTime: (json['totalFocusTime'] as num).toInt(),
  percentage: (json['percentage'] as num).toDouble(),
  taskDistribution:
      (json['taskDistribution'] as List<dynamic>)
          .map((e) => TaskDistributionDto.fromJson(e as Map<String, dynamic>))
          .toList(),
);

Map<String, dynamic> _$CategoryDistributionDtoToJson(
  _CategoryDistributionDto instance,
) => <String, dynamic>{
  'categoryId': instance.categoryId,
  'categoryName': instance.categoryName,
  'totalFocusTime': instance.totalFocusTime,
  'percentage': instance.percentage,
  'taskDistribution': instance.taskDistribution,
};

_TaskDistributionDto _$TaskDistributionDtoFromJson(Map<String, dynamic> json) =>
    _TaskDistributionDto(
      taskName: json['taskName'] as String,
      totalFocusTime: (json['totalFocusTime'] as num).toInt(),
      percentage: (json['percentage'] as num).toDouble(),
    );

Map<String, dynamic> _$TaskDistributionDtoToJson(
  _TaskDistributionDto instance,
) => <String, dynamic>{
  'taskName': instance.taskName,
  'totalFocusTime': instance.totalFocusTime,
  'percentage': instance.percentage,
};

_DailyActivityDistributionDto _$DailyActivityDistributionDtoFromJson(
  Map<String, dynamic> json,
) => _DailyActivityDistributionDto(
  categoryId: json['categoryId'] as String,
  categoryName: json['categoryName'] as String,
  totalFocusTime: (json['totalFocusTime'] as num).toInt(),
);

Map<String, dynamic> _$DailyActivityDistributionDtoToJson(
  _DailyActivityDistributionDto instance,
) => <String, dynamic>{
  'categoryId': instance.categoryId,
  'categoryName': instance.categoryName,
  'totalFocusTime': instance.totalFocusTime,
};

_DailyActivityDto _$DailyActivityDtoFromJson(Map<String, dynamic> json) =>
    _DailyActivityDto(
      date: (json['date'] as num).toInt(),
      categoryDistribution:
          (json['categoryDistribution'] as List<dynamic>)
              .map(
                (e) => DailyActivityDistributionDto.fromJson(
                  e as Map<String, dynamic>,
                ),
              )
              .toList(),
    );

Map<String, dynamic> _$DailyActivityDtoToJson(_DailyActivityDto instance) =>
    <String, dynamic>{
      'date': instance.date,
      'categoryDistribution': instance.categoryDistribution,
    };

_GetStatsByPeriodResponseDto _$GetStatsByPeriodResponseDtoFromJson(
  Map<String, dynamic> json,
) => _GetStatsByPeriodResponseDto(
  totalSessions: (json['totalSessions'] as num).toInt(),
  totalBreaks: (json['totalBreaks'] as num).toInt(),
  totalFocusTime: (json['totalFocusTime'] as num).toInt(),
  totalBreakTime: (json['totalBreakTime'] as num).toInt(),
  focusPauseRatio: (json['focusPauseRatio'] as num).toDouble(),
  mostConcentratedPeriod: json['mostConcentratedPeriod'] as String,
  lessConcentratedPeriod: json['lessConcentratedPeriod'] as String,
  concentrationDistribution:
      (json['concentrationDistribution'] as List<dynamic>)
          .map((e) => (e as num).toInt())
          .toList(),
  categoryDistribution:
      (json['categoryDistribution'] as List<dynamic>)
          .map(
            (e) => CategoryDistributionDto.fromJson(e as Map<String, dynamic>),
          )
          .toList(),
  dailyActivity:
      (json['dailyActivity'] as List<dynamic>)
          .map((e) => DailyActivityDto.fromJson(e as Map<String, dynamic>))
          .toList(),
);

Map<String, dynamic> _$GetStatsByPeriodResponseDtoToJson(
  _GetStatsByPeriodResponseDto instance,
) => <String, dynamic>{
  'totalSessions': instance.totalSessions,
  'totalBreaks': instance.totalBreaks,
  'totalFocusTime': instance.totalFocusTime,
  'totalBreakTime': instance.totalBreakTime,
  'focusPauseRatio': instance.focusPauseRatio,
  'mostConcentratedPeriod': instance.mostConcentratedPeriod,
  'lessConcentratedPeriod': instance.lessConcentratedPeriod,
  'concentrationDistribution': instance.concentrationDistribution,
  'categoryDistribution': instance.categoryDistribution,
  'dailyActivity': instance.dailyActivity,
};
