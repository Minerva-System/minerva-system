import 'package:flutter_neumorphic/flutter_neumorphic.dart';

class MinervaCard extends StatelessWidget {
  final EdgeInsets padding;
  final Widget? child;

  const MinervaCard({
    Key? key,
    this.padding = const EdgeInsets.symmetric(vertical: 10.0, horizontal: 50.0),
    this.child,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Neumorphic(
      // TODO: Inherit style
      style: NeumorphicStyle(
        shape: NeumorphicShape.flat,
        boxShape: NeumorphicBoxShape.roundRect(BorderRadius.circular(12)),
        depth: 6,
        intensity: 0.6,
        lightSource: LightSource.topLeft,
        color: const Color(0xFFDFE0E7),
      ),
      padding: padding,
      child: Align(alignment: Alignment.center, child: child),
    );
  }
}
