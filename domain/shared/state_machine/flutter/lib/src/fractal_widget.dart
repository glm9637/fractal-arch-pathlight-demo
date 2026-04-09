import 'package:flutter/material.dart';

class FractalWidget extends StatefulWidget {
  final Future<void> Function() onInit;
  final Widget child;
  final VoidCallback? onDispose;

  const FractalWidget({
    super.key,
    required this.onInit,
    required this.child,
    this.onDispose,
  });

  @override
  State<FractalWidget> createState() => _FractalWidgetState();
}

class _FractalWidgetState extends State<FractalWidget> {
  bool _isInitialized = false;
  String? _error;

  @override
  void initState() {
    super.initState();
    _bootSystem();
  }

  Future<void> _bootSystem() async {
    try {
      await widget.onInit();
      if (mounted) {
        setState(() => _isInitialized = true);
      }
    } catch (e) {
      if (mounted) {
        setState(() => _error = e.toString());
      }
    }
  }

  @override
  void dispose() {
    widget.onDispose?.call();
    // Note: Rust side usually cleans up automatically when
    // the StreamSinks are dropped by Flutter's StreamBuilders.
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (_error != null) {
      return Center(child: Text("Domain Init Error: $_error"));
    }
    if (!_isInitialized) {
      return const Center(child: CircularProgressIndicator());
    }

    return widget.child;
  }
}
