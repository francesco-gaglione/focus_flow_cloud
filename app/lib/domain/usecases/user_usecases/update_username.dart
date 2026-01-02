import '../../repositories/auth_repository.dart';

class UpdateUsername {
  final AuthRepository repository;

  UpdateUsername(this.repository);

  Future<void> call(String newUsername) {
    return repository.updateUsername(newUsername);
  }
}
