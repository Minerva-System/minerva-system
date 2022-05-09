import 'package:get/get.dart';
import 'package:get/get_navigation/src/routes/get_route.dart';
import 'package:minerva_frontend/modules/login/login_page.dart';

class LoginRouters {
  LoginRouters._();

  static final routers = <GetPage>[
    GetPage(
      name: '/:tenant/',
      page: () => const LoginPage(),
    ),
  ];
}
