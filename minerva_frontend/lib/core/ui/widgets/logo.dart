import 'package:flutter_neumorphic/flutter_neumorphic.dart';

class MinervaLogo extends StatelessWidget {
  final EdgeInsets? padding;
  final double scale;

  const MinervaLogo({
    Key? key,
    this.padding,
    this.scale = 1.0,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: padding,
      child: Transform(
        alignment: FractionalOffset.center,
        transform: Matrix4.identity()..scale(scale),
        child: Center(
          child: Neumorphic(
            style: const NeumorphicStyle(
              shape: NeumorphicShape.concave,
              boxShape: NeumorphicBoxShape.circle(),
              depth: -10,
              color: Color(0xFFDEE1ED),
            ),
            padding: const EdgeInsets.all(14.0),
            child: Image.asset(
              'assets/images/logo.png',
              fit: BoxFit.fitWidth,
            ),
          ),
        ),
      ),
    );
  }
}
