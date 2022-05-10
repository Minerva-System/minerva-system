import 'package:flutter_neumorphic/flutter_neumorphic.dart';

class MinervaLogo extends StatelessWidget {
  final EdgeInsets? padding;

  const MinervaLogo({
      Key? key,
      this.padding,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: padding,
      child: Neumorphic(
        style: const NeumorphicStyle(
          shape: NeumorphicShape.concave,
          boxShape: NeumorphicBoxShape.circle(),
          depth: -10,
          color: Color(0xFFDEE1ED),
        ),
        child: Image.asset('assets/images/logo.png'),
        padding: const EdgeInsets.all(12.0),
      ),
    );
  }
}
