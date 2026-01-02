class UpdatePasswordDto {
  final String oldPassword;
  final String newPassword;

  UpdatePasswordDto({required this.oldPassword, required this.newPassword});

  Map<String, dynamic> toJson() => {
    'old_password': oldPassword,
    'new_password': newPassword,
  };
}

class UpdateUsernameDto {
  final String newUsername;

  UpdateUsernameDto({required this.newUsername});

  Map<String, dynamic> toJson() => {'new_username': newUsername};
}

class UserInfoResponseDto {
  final String id;
  final String username;
  final String role;

  UserInfoResponseDto({
    required this.id,
    required this.username,
    required this.role,
  });

  factory UserInfoResponseDto.fromJson(Map<String, dynamic> json) {
    return UserInfoResponseDto(
      id: json['id'] as String,
      username: json['username'] as String,
      role: json['role'] as String,
    );
  }
}

class CreateUserDto {
  final String username;
  final String password;

  CreateUserDto({required this.username, required this.password});

  Map<String, dynamic> toJson() => {'username': username, 'password': password};
}
