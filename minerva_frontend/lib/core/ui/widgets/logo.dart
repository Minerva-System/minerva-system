import 'package:flutter_neumorphic/flutter_neumorphic.dart';

class MinervaLogo extends StatelessWidget {
  const MinervaLogo({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.only(bottom: 50.0),
      child: Neumorphic(
        style: const NeumorphicStyle(
          shape: NeumorphicShape.concave,
          boxShape: NeumorphicBoxShape.circle(),
          depth: -10,
          color: Color(0xFFDEE1ED),
        ),
        child: Image.asset('assets/images/logo.png'),
      ),
    );
  }
}
