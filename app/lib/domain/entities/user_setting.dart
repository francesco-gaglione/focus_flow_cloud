import 'package:equatable/equatable.dart';

class UserSetting extends Equatable {
  final String key;
  final String value;

  const UserSetting({required this.key, required this.value});

  UserSetting copyWith({String? key, String? value}) {
    return UserSetting(key: key ?? this.key, value: value ?? this.value);
  }

  @override
  List<Object?> get props => [key, value];
}
