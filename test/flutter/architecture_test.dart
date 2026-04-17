import 'dart:io';

import 'package:dart_arch_test/dart_arch_test.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  late DependencyGraph graph;
  late List<String> domains;

  setUpAll(() async {
    graph = await Collector.buildGraph('./');
    domains = loadDomains();
  });

  test('Domains must only import their specific rust bridge file', () {
    for (final entry in graph.entries) {
      final fileUri = entry.key;
      final imports = entry.value;
      validateRustBindings(fileUri, imports, domains);
    }
  });
}

void validateRustBindings(
  String fileUri,
  Set<String> imports,
  List<String> domains,
) {
  for (var domain in domains) {
    if (fileUri.startsWith('package:${domain}_frontend/')) {
      for (final importUri in imports) {
        if (importUri.startsWith('package:flutter_rust/')) {
          if (importUri != 'package:flutter_rust/$domain.dart') {
            fail(
              'Architecture Violation Found!\n'
              'File: $fileUri\n'
              'Illegal Import: $importUri\n',
            );
          }
        }
      }
    }
  }
}

List<String> loadDomains() {
  final domainDir = Directory('domain');
  List<String> result = [];

  if (domainDir.existsSync()) {
    for (final entity in domainDir.listSync()) {
      if (entity is Directory) {
        // Safely extract the folder name from the URI
        final folderName = entity.uri.pathSegments
            .where((s) => s.isNotEmpty)
            .last;

        // Exclude the 'shared' folder and any hidden directories (.git, etc.)
        if (folderName != 'shared' && !folderName.startsWith('.')) {
          result.add(folderName);
        }
      }
    }
  }
  return result;
}
