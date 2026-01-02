class LoginDto {
  final String username;
  final String password;

  LoginDto({required this.username, required this.password});

  Map<String, dynamic> toJson() => {
    'username': username,
    'password': password,
  };
}

class LoginResponseDto {
  final String token;
  final String refreshToken;

  LoginResponseDto({required this.token, required this.refreshToken});

  factory LoginResponseDto.fromJson(Map<String, dynamic> json) {
    return LoginResponseDto(
      token: json['token'] as String,
      refreshToken: json['refresh_token'] as String,
    );
  }
}

class RefreshDto {
  final String refreshToken;

  RefreshDto({required this.refreshToken});

  Map<String, dynamic> toJson() => {
    'refresh_token': refreshToken,
  };
}

class RefreshResponseDto {
  final String token;
  final String refreshToken;

  RefreshResponseDto({required this.token, required this.refreshToken});

  factory RefreshResponseDto.fromJson(Map<String, dynamic> json) {
    return RefreshResponseDto(
      token: json['token'] as String,
      refreshToken: json['refresh_token'] as String,
    );
  }
}

class LogoutResponseDto {
  final String message;

  LogoutResponseDto({required this.message});

  factory LogoutResponseDto.fromJson(Map<String, dynamic> json) {
    return LogoutResponseDto(
      message: json['message'] as String,
    );
  }
}
