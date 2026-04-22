import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_rust/auth.dart';
import 'package:oidc/oidc.dart';

class AuthStateNotifier extends ChangeNotifier {
  bool _isLoggedIn = false;
  bool get isLoggedIn => _isLoggedIn;

  StreamSubscription? _rustSubscription;
  StreamSubscription? _oidcSubscription;
  StreamSubscription? _refreshSubscription;

  void startListening(OidcUserManager manager) {
    _rustSubscription ??= watchLoggedIn().listen((event) {
      debugPrint('Rust auth state changed: ${event.loggedIn}');
      if (_isLoggedIn != event.loggedIn) {
        _isLoggedIn = event.loggedIn;
        notifyListeners();
      }
    });

    _refreshSubscription ??= watchNeedsRefresh().listen((event) async {
      if (event.needsRefresh) {
        debugPrint(
          'Rust requested a token refresh! Attempting silent refresh...',
        );

        try {
          final newUser = await manager.refreshToken();

          if (newUser == null) {
            debugPrint('Refresh failed (returned null). Logging out...');
          }
        } catch (e) {
          debugPrint('Fatal error during token refresh: $e');
          // await _handleLogout(manager);
        }
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
