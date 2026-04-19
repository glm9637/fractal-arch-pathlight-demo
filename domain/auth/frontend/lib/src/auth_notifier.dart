import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_rust/auth.dart';
import 'package:oidc/oidc.dart';

class AuthStateNotifier extends ChangeNotifier {
  bool _isLoggedIn = false;
  bool get isLoggedIn => _isLoggedIn;

  StreamSubscription? _rustSubscription;
  StreamSubscription? _oidcSubscription;

  void startListening(OidcUserManager manager) {
    _rustSubscription ??= watchLoggedIn().listen((event) {
      debugPrint('Rust auth state changed: ${event.loggedIn}');
      if (_isLoggedIn != event.loggedIn) {
        _isLoggedIn = event.loggedIn;
        notifyListeners();
      }
    });

    _oidcSubscription ??= manager.userChanges().listen((OidcUser? user) {
      if (user != null && user.token.accessToken != null) {
        debugPrint('Dart grabbed tokens, sending to Rust...');

        dispatchSetTokensCommand(
          command: SetTokensCommand(
            accessToken: user.token.accessToken!,
            refreshToken: user.token.refreshToken ?? "",
          ),
        );
      } else {
        // Optional: If the user logs out from the Dart side, tell Rust to wipe tokens
        debugPrint('Dart OIDC user is null. Wiping Rust state...');
        // dispatchClearTokensCommand();
      }
    });
  }

  @override
  void dispose() {
    _rustSubscription?.cancel();
    _oidcSubscription?.cancel();
    super.dispose();
  }
}
