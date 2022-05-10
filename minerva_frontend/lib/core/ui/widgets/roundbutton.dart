import 'package:flutter_neumorphic/flutter_neumorphic.dart';

class MinervaRoundButton extends StatelessWidget {
  final void Function()? onPressed;
  final Widget? child;
  final EdgeInsets? margin;
  final EdgeInsets? padding;

  const MinervaRoundButton({
    Key? key,
    this.onPressed,
    this.child,
    this.margin,
    this.padding,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return NeumorphicButton(
      // TODO: offload?
      margin: margin,
      onPressed: onPressed,
      child: child,
      padding: padding,
      style: const NeumorphicStyle(
        shape: NeumorphicShape.concave,
        boxShape: NeumorphicBoxShape.circle(),
        depth: 5,
        intensity: 0.6,
        color: Color(0xFFDFE0E6),
      ),
    );
  }
}
