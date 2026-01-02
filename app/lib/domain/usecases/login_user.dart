import 'package:focus_flow_app/adapters/dtos/auth_dtos.dart';
import '../repositories/auth_repository.dart';

class LoginUser {
  final AuthRepository repository;

  LoginUser(this.repository);

  Future<LoginResponseDto> call(String username, String password) {
    return repository.login(username, password);
  }
}
