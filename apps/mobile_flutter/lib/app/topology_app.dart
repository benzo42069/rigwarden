import 'package:flutter/material.dart';
import 'package:rigwarden/features/session/session_shell.dart';

/// Minimal app-facing entry point for the adaptive session shell.
class TopologyApp extends StatelessWidget {
  const TopologyApp({required this.session, super.key});

  final SessionShellModel session;

  @override
  Widget build(BuildContext context) {
    return SessionShell(session: session);
  }
}
