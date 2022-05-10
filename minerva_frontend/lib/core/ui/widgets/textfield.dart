import 'package:flutter_neumorphic/flutter_neumorphic.dart';

class MinervaTextField extends StatelessWidget {
  final String label;
  final TextEditingController? controller;
  final bool obscureText;
  final FormFieldValidator<String>? validator;
  final ValueChanged<String>? onChange;
  final EdgeInsets? padding;

  const MinervaTextField({
    Key? key,
    required this.label,
    this.controller,
    this.validator,
    this.onChange,
    this.obscureText = false,
    this.padding,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Material(
      color: const Color(0x00FFFFFF),
      child: Container(
        padding: padding,
        child: Neumorphic(
          // TODO: Export this style.
          style: NeumorphicStyle(
            shape: NeumorphicShape.flat,
            boxShape: NeumorphicBoxShape.roundRect(BorderRadius.circular(50)),
            depth: -2,
            intensity: 1.5,
            color: const Color(0xFFDEE1ED),
          ),
          child: Padding(  
            padding:
                const EdgeInsets.symmetric(horizontal: 20.0, vertical: 5.0),
            child: TextFormField(
              validator: validator,
              obscureText: obscureText,
              onChanged: onChange,
              controller: controller,
              decoration: InputDecoration(
                isDense: true,
                labelText: label,
                labelStyle: const TextStyle(
                  color: Color(0xFF9A9A9A),
                  backgroundColor: Colors.transparent,
                ),
                errorStyle: const TextStyle(
                  color: Colors.redAccent,
                  backgroundColor: Colors.transparent,
                ),
                filled: false,
                border: InputBorder.none,
              ),
            ),
          ),
        ),
      ),
    );
  }
}
