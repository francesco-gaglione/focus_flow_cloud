part of 'version_cubit.dart';

abstract class VersionState {}

class VersionInitial extends VersionState {}

class VersionChecking extends VersionState {}

class VersionCompatible extends VersionState {}

class VersionIncompatible extends VersionState {}
