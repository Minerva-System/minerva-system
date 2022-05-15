import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'package:get/get.dart';
import 'package:minerva_frontend/core/ui/widgets/textfield.dart';
import 'package:minerva_frontend/core/ui/widgets/card.dart';
import 'package:minerva_frontend/core/ui/widgets/background.dart';
import 'package:minerva_frontend/core/ui/widgets/roundbutton.dart';
import 'package:minerva_frontend/core/ui/widgets/logo.dart';
import 'package:validatorless/validatorless.dart';
import 'package:minerva_frontend/modules/login/login_controller.dart';
import 'package:minerva_frontend/core/ui/minerva_state.dart';
import 'package:fluttericon/mfg_labs_icons.dart';

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
        child: Container(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: <Widget>[
              const MinervaLogo(
                padding: EdgeInsets.only(bottom: 20.0),
              ),
              Form(
                key: _formKey,
                child: Column(children: <Widget>[
                  const MinervaTextField(
                    label: "Login",
                    padding: EdgeInsets.only(bottom: 20.0),
                  ),
                  MinervaTextField(
                    padding: const EdgeInsets.only(bottom: 20.0),
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
                padding: const EdgeInsets.all(20),
                onPressed: () {
                  final formValid = _formKey.currentState?.validate() ?? false;
                  if (formValid) {
                    // TODO: Perform actual login
                    var tenant = "/${Get.parameters['tenant'] ?? 'minerva'}";
                    Get.toNamed("$tenant/main");
                  }
                },
                child: const Icon(
                  MfgLabs.right,
                  size: 30,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
