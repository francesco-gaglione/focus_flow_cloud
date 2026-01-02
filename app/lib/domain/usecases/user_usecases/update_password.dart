import '../../repositories/auth_repository.dart';

class UpdatePassword {
  final AuthRepository repository;

  UpdatePassword(this.repository);

  Future<void> call(String oldPassword, String newPassword) {
    return repository.updatePassword(oldPassword, newPassword);
  }
}
