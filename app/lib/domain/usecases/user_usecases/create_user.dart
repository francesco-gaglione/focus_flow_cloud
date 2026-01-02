import '../../repositories/auth_repository.dart';

class CreateUser {
  final AuthRepository _repository;

  CreateUser(this._repository);

  Future<void> call(String username, String password) async {
    return await _repository.createUser(username, password);
  }
}
