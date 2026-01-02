import 'package:equatable/equatable.dart';

class NoteTemplate extends Equatable {
  final String id;
  final String name;
  final String content;

  const NoteTemplate({
    required this.id,
    required this.name,
    required this.content,
  });

  NoteTemplate copyWith({
    String? id,
    String? name,
    String? content,
  }) {
    return NoteTemplate(
      id: id ?? this.id,
      name: name ?? this.name,
      content: content ?? this.content,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'name': name,
      'content': content,
    };
  }

  factory NoteTemplate.fromJson(Map<String, dynamic> json) {
    return NoteTemplate(
      id: json['id'] as String,
      name: json['name'] as String,
      content: json['content'] as String,
    );
  }

  @override
  List<Object?> get props => [id, name, content];
}
