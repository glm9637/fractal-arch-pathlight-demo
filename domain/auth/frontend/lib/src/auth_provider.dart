import 'package:flutter/material.dart';
import 'package:oidc/oidc.dart';

class AuthManagerProvider extends InheritedWidget {
  final OidcUserManager manager;

  const AuthManagerProvider({
    super.key,
    required this.manager,
    required super.child,
  });

  static OidcUserManager of(BuildContext context) {
    final provider = context
        .dependOnInheritedWidgetOfExactType<AuthManagerProvider>();
    if (provider == null) {
      throw StateError('AuthManagerProvider not found in widget tree');
    }
    return provider.manager;
  }

  @override
  bool updateShouldNotify(AuthManagerProvider oldWidget) {
    return manager != oldWidget.manager;
  }
}
