import 'package:equatable/equatable.dart';

class Task extends Equatable {
  final String id;
  final String name;
  final String? description;
  final String? categoryId;
  final int? scheduledDate;
  final int? completedAt;
  final DateTime createdAt;
  final DateTime updatedAt;

  const Task({
    required this.id,
    required this.name,
    this.description,
    this.categoryId,
    this.scheduledDate,
    this.completedAt,
    required this.createdAt,
    required this.updatedAt,
  });

  Task copyWith({
    String? id,
    String? name,
    String? description,
    String? categoryId,
    int? scheduledDate,
    int? completedAt,
    DateTime? createdAt,
    DateTime? updatedAt,
  }) {
    return Task(
      id: id ?? this.id,
      name: name ?? this.name,
      description: description ?? this.description,
      categoryId: categoryId ?? this.categoryId,
      scheduledDate: scheduledDate ?? this.scheduledDate,
      completedAt: completedAt ?? this.completedAt,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
    );
  }

  bool get isCompleted => completedAt != null;

  bool get isOrphan => categoryId == null;

  @override
  String toString() {
    return 'Task(id: $id, name: $name, description: $description, categoryId: $categoryId, scheduledDate: $scheduledDate, completedAt: $completedAt, createdAt: $createdAt, updatedAt: $updatedAt)';
  }

  @override
  List<Object?> get props => [
    id,
    name,
    description,
    categoryId,
    scheduledDate,
    completedAt,
    createdAt,
    updatedAt,
  ];
}
