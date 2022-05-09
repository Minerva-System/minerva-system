import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'package:minerva_frontend/core/ui/widgets/minerva_textfield.dart';
import 'package:minerva_frontend/core/ui/widgets/minerva_card.dart';
import 'package:minerva_frontend/core/ui/widgets/minerva_background.dart';
import 'package:minerva_frontend/core/ui/widgets/minerva_roundbutton.dart';
import 'package:minerva_frontend/core/ui/widgets/minerva_logo.dart';
import 'package:validatorless/validatorless.dart';
import 'package:minerva_frontend/modules/login/login_controller.dart';
import 'package:minerva_frontend/core/ui/minerva_state.dart';

class LoginPage extends StatefulWidget {
  const LoginPage({Key? key}) : super(key: key);

  @override
  State<LoginPage> createState() => _LoginPageState();
}

class _LoginPageState extends MinervaState<LoginPage, LoginController> {
  final _formKey = GlobalKey<FormState>();
  final _passwordEC = TextEditingController();

  @override
  void dispose() {
    _passwordEC.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return MinervaBackground(
      child: MinervaCard(
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: <Widget>[
              const MinervaLogo(),
              Form(
                child: Column(children: <Widget>[
                  const MinervaTextField(label: "Login"),
                  MinervaTextField(
                    label: "Senha",
                    obscureText: true,
                    controller: _passwordEC,
                    validator: Validatorless.multiple([
                      Validatorless.required('Senha obrigatória'),
                      Validatorless.min(
                          6, 'Senha deve ter pelo menos seis caracteres'),
                    ]),
                  ),
                ]),
              ),
              MinervaRoundButton(
                onPressed: () {
                  // TODO: This is not working...
                  final formValid = _formKey.currentState?.validate() ?? false;
                  if (formValid) {
                    _passwordEC.clear();
                  }
                },
                child: const Icon(
                  Icons.arrow_forward,
                  size: 50,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
