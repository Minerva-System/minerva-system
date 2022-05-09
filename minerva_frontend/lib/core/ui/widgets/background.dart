import 'package:flutter/material.dart';

class MinervaBackground extends StatelessWidget {
  final Widget? child;
  final EdgeInsets padding;

  const MinervaBackground({
    Key? key,
    this.child,
    this.padding = const EdgeInsets.symmetric(vertical: 90.0, horizontal: 100.0),
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: padding,
      color: const Color(0xFFDADEEF),
      child: child,
    );
  }
}
