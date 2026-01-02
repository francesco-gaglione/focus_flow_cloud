import '../../repositories/auth_repository.dart';
import '../../../adapters/dtos/user_dtos.dart';

class GetUserInfo {
  final AuthRepository repository;

  GetUserInfo(this.repository);

  Future<UserInfoResponseDto> call() {
    return repository.getUserInfo();
  }
}
