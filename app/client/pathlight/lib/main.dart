import 'package:auth_frontend/main.dart';
import 'package:flutter/material.dart';

import 'package:flutter_rust/main.dart';

import 'auth_guard.dart';
import 'router.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  await initLogging();
  final authState = AuthStateNotifier();
  final appRouter = AppRouter(authGuard: AuthGuard(authState));
  runApp(PathlightApp(authStateNotifier: authState, router: appRouter));
}

class PathlightApp extends StatelessWidget {
  final AuthStateNotifier authStateNotifier;
  final AppRouter router;

  const PathlightApp({
    super.key,
    required this.authStateNotifier,
    required this.router,
  });

  @override
  Widget build(BuildContext context) {
    // AuthPage wraps the single MaterialApp.router instance
    return AuthPage(
      authState: authStateNotifier,
      child: MaterialApp.router(
        title: 'Pathlight',
        theme: ThemeData(
          colorScheme: ColorScheme.fromSeed(seedColor: Colors.blueGrey),
          useMaterial3: true,
        ),
        routerConfig: router.config(reevaluateListenable: authStateNotifier),
      ),
    );
  }
}
