import 'package:flutter/material.dart';

class MinervaBackground extends StatelessWidget {
  final Widget? child;
  final EdgeInsets padding;
  final Color backgroundColor;

  const MinervaBackground({
    Key? key,
    this.child,
    this.padding =
        const EdgeInsets.symmetric(vertical: 90.0, horizontal: 100.0),
    this.backgroundColor = const Color(0xFFDADEEF),
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      // padding: padding,
      backgroundColor: backgroundColor,
      body: LayoutBuilder(builder: (_, constraints) {
        var maxWidth = 600.0;
        var maxHeight = 500.0;

        // If maxWidth surpasses screen size, also adjust inner
        // child so that the card fits the constrained size
        if (maxWidth >= constraints.maxWidth) {
          maxHeight = constraints.maxHeight;
        }

        return Center(
          child: ConstrainedBox(
            constraints: BoxConstraints(
              maxWidth: maxWidth,
              maxHeight: maxHeight,
            ),
            child: IntrinsicHeight(child: child),
          ),
        );
      }),
    );
  }
}
