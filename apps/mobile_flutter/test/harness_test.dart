import 'package:flutter_test/flutter_test.dart';
import 'package:rigwarden/main.dart';

void main() {
  testWidgets('minimal app mounts', (tester) async {
    await tester.pumpWidget(const RigWardenApp());

    expect(find.byType(RigWardenApp), findsOneWidget);
  });
}
