import 'package:flutter/material.dart';

import 'package:todo_frontend/frontend.dart';
import 'package:flutter_rust/main.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  await initLogging();
  runApp(const PathlightApp());
}

class PathlightApp extends StatelessWidget {
  const PathlightApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Pathlight',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.blueGrey),
        useMaterial3: true,
      ),
      home: TodoFeature(),
    );
  }
}
