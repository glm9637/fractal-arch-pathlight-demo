import 'package:auth_frontend/src/auth_helper.dart';
import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';

import 'auth_provider.dart';

@RoutePage()
class AuthScreen extends StatefulWidget {
  const AuthScreen({super.key});

  @override
  State<AuthScreen> createState() => _AuthScreenState();
}

class _AuthScreenState extends State<AuthScreen> {
  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Auth Domain (Rust Powered)')),
      body: Center(child: Text("Welcome to Pathlight, log in now")),
      floatingActionButton: FloatingActionButton(
        onPressed: () async {
          final manager = AuthManagerProvider.of(context);
          await performLogin(manager);
        },
        child: const Icon(Icons.login),
      ),
    );
  }
}
