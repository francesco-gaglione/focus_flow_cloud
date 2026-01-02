enum SessionType {
  work('Work'),
  shortBreak('ShortBreak'),
  longBreak('LongBreak');

  final String value;
  const SessionType(this.value);

  static SessionType fromString(String value) {
    switch (value) {
      case 'Work':
        return SessionType.work;
      case 'ShortBreak':
        return SessionType.shortBreak;
      case 'LongBreak':
        return SessionType.longBreak;
      default:
        throw ArgumentError('Invalid SessionType: $value');
    }
  }
}

class FocusSession {
  final String id;
  final SessionType sessionType;
  final int startedAt;
  final int? endedAt;
  final int? actualDuration;
  final String? taskId;
  final String? categoryId;
  final int? concentrationScore;
  final String? notes;
  final DateTime createdAt;

  FocusSession({
    required this.id,
    required this.sessionType,
    required this.startedAt,
    this.endedAt,
    this.actualDuration,
    this.taskId,
    this.categoryId,
    this.concentrationScore,
    this.notes,
    required this.createdAt,
  });

  FocusSession copyWith({
    String? id,
    SessionType? sessionType,
    int? startedAt,
    int? endedAt,
    int? actualDuration,
    String? taskId,
    String? categoryId,
    int? concentrationScore,
    String? notes,
    DateTime? createdAt,
  }) {
    return FocusSession(
      id: id ?? this.id,
      sessionType: sessionType ?? this.sessionType,
      startedAt: startedAt ?? this.startedAt,
      endedAt: endedAt ?? this.endedAt,
      actualDuration: actualDuration ?? this.actualDuration,
      taskId: taskId ?? this.taskId,
      categoryId: categoryId ?? this.categoryId,
      concentrationScore: concentrationScore ?? this.concentrationScore,
      notes: notes ?? this.notes,
      createdAt: createdAt ?? this.createdAt,
    );
  }

  @override
  String toString() {
    return 'FocusSession(id: $id, sessionType: $sessionType, startedAt: $startedAt, endedAt: $endedAt, actualDuration: $actualDuration, taskId: $taskId, categoryId: $categoryId, concentrationScore: $concentrationScore, notes: $notes, createdAt: $createdAt)';
  }
}
