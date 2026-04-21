import 'package:flutter/material.dart';
import 'package:flutter_rust/auth.dart';
import 'package:oidc/oidc.dart';

import 'auth_helper.dart';
import 'auth_notifier.dart';
import 'auth_provider.dart';

class AuthPage extends StatefulWidget {
  final Widget child; // This is your MaterialApp.router
  final AuthStateNotifier authState;

  const AuthPage({super.key, required this.child, required this.authState});

  @override
  State<AuthPage> createState() => _AuthPageState();
}

class _AuthPageState extends State<AuthPage> {
  bool _isInitDone = false;
  OidcUserManager? _manager; // Add the manager to state

  @override
  void initState() {
    super.initState();
    _bootAuthSystem();
  }

  Future<void> _bootAuthSystem() async {
    await initAuthSystem(config: AuthSystemConfig(baseUrl: "todo"));

    var manager = await initAuth();

    widget.authState.startListening(manager);

    if (mounted) {
      setState(() {
        _manager = manager; // Save it
        _isInitDone = true;
      });
    }
  }

  @override
  void dispose() {
    disposeAuthSystem();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (!_isInitDone || _manager == null) {
      // DO NOT USE MaterialApp HERE!
      // We use raw widgets so Flutter doesn't hijack the browser URL.
      return const Directionality(
        textDirection: TextDirection.ltr,
        child: ColoredBox(
          color: Colors.white,
          child: Center(
            child: CircularProgressIndicator(color: Colors.blueGrey),
          ),
        ),
      );
    }

    return AuthManagerProvider(manager: _manager!, child: widget.child);
  }
}
