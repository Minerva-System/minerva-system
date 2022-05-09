import 'package:flutter_neumorphic/flutter_neumorphic.dart';

class MinervaRoundButton extends StatelessWidget {
  final void Function()? onPressed;
  final Widget? child;

  const MinervaRoundButton({
    Key? key,
    this.onPressed,
    this.child,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return NeumorphicButton(
      // TODO: offload?
      margin: const EdgeInsets.only(top: 12),
      onPressed: onPressed,
      child: child,
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
