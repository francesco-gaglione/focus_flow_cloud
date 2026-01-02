import 'package:flutter_bloc/flutter_bloc.dart';
import '../../../../domain/usecases/user_usecases/get_user_info.dart';
import '../../../../domain/usecases/user_usecases/update_password.dart';
import '../../../../domain/usecases/user_usecases/update_username.dart';
import '../../../../domain/usecases/user_usecases/create_user.dart';
import 'account_state.dart';

class AccountCubit extends Cubit<AccountState> {
  final UpdatePassword updatePassword;
  final UpdateUsername updateUsername;
  final GetUserInfo getUserInfo;
  final CreateUser createUserUseCase;

  AccountCubit({
    required this.updatePassword,
    required this.updateUsername,
    required this.getUserInfo,
    required this.createUserUseCase,
  }) : super(AccountInitial());

  Future<void> loadUserInfo() async {
    emit(AccountLoading());
    try {
      final userInfo = await getUserInfo();
      emit(AccountLoaded(userInfo));
    } catch (e) {
      emit(AccountError(e.toString()));
    }
  }

  Future<void> changePassword(String oldPassword, String newPassword) async {
    // Keep loading state or show loading indicator??
    // Ideally we shouldn't replace AccountLoaded with Loading if we want to keep showing data
    // But for simplicity let's just emit loading.
    emit(AccountLoading());
    try {
      await updatePassword(oldPassword, newPassword);
      emit(const AccountSuccess('Password updated successfully'));
      // Reload info? Not strictly needed for password.
      await loadUserInfo();
    } catch (e) {
      emit(AccountError(e.toString()));
      // If error, try to reload info to restore UI state
      loadUserInfo();
    }
  }

  Future<void> changeUsername(String newUsername) async {
    emit(AccountLoading());
    try {
      await updateUsername(newUsername);
      emit(const AccountSuccess('Username updated successfully'));
      await loadUserInfo();
    } catch (e) {
      emit(AccountError(e.toString()));
      loadUserInfo();
    }
  }

  Future<void> createUser(String username, String password) async {
    emit(AccountLoading());
    try {
      await createUserUseCase(username, password);
      emit(const AccountSuccess('User created successfully'));
      await loadUserInfo();
    } catch (e) {
      emit(AccountError(e.toString()));
      loadUserInfo();
    }
  }
}
