import 'package:flutter_test/flutter_test.dart';
import 'package:rigwarden/core/bridge/generated/api.dart';
import 'package:rigwarden/core/bridge/generated/frb_generated.dart';

void main() {
  test('typed device identity round trips from Rust', () async {
    await RustLib.init();

    final identity = await readFixtureDeviceIdentity();

    expect(identity.family, 'AM4');
    expect(identity.model, 'AM4');
    expect(identity.firmware, '1.00');
    expect(identity.transportEndpoint, 'fixture://am4');
  });
}
