import 'package:equatable/equatable.dart';
import '../../../adapters/dtos/user_dtos.dart';

abstract class AccountState extends Equatable {
  const AccountState();

  @override
  List<Object?> get props => [];
}

class AccountInitial extends AccountState {}

class AccountLoading extends AccountState {}

class AccountLoaded extends AccountState {
    final UserInfoResponseDto userInfo;
    
    const AccountLoaded(this.userInfo);

    @override
    List<Object?> get props => [userInfo];
}

class AccountSuccess extends AccountState {
    final String message;
    const AccountSuccess(this.message);
    
    @override
    List<Object?> get props => [message];
}

class AccountError extends AccountState {
  final String message;
  const AccountError(this.message);

  @override
  List<Object?> get props => [message];
}
