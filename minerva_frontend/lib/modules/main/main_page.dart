import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'package:fluttericon/mfg_labs_icons.dart';
import 'package:minerva_frontend/core/ui/widgets/background.dart';
import 'package:minerva_frontend/core/ui/widgets/logo.dart';
import 'package:collapsible_sidebar/collapsible_sidebar.dart';

// https://pub.dev/packages/collapsible_sidebar

class MainPage extends StatelessWidget {
  const MainPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      child: Scaffold(
        appBar: AppBar(
          flexibleSpace: const Spacer(flex: 2),
          leading: const MinervaLogo(scale: 1),
          leadingWidth: 100,
          title: const Text(
            "Início",
            textAlign: TextAlign.left,
            style: TextStyle(
              fontFamily: 'nunitosans',
              color: Color(0xFF2B2B2B),
              fontSize: 40,
            ),
          ),
          elevation: 0,
          backgroundColor: const Color(0xFFE5E5E5),
        ),
        body: MinervaBackground(
          alignment: Alignment.topLeft,
          backgroundColor: const Color(0xFFE5E5E5),
          child: Container(
            padding: const EdgeInsets.only(left: 10, top: 10),
            child: CollapsibleSidebar(
              isCollapsed: true,
              onHoverPointer: SystemMouseCursors.contextMenu,
              backgroundColor: const Color(0xFFE5E5E5),
              selectedIconBox: const Color(0xFFE5E5E5),
              unselectedTextColor: const Color(0xFF9A9A9A),
              selectedTextColor: const Color(0xFF2B2B2B),
              selectedIconColor: const Color(0xFF990048),
              unselectedIconColor: const Color(0xFF2B2B2B),
              textStyle: const TextStyle(fontFamily: 'nunitosans'),
              sidebarBoxShadow: const [
                BoxShadow(
                  color: Colors.black38,
                  blurRadius: 10,
                  spreadRadius: 0.01,
                  offset: Offset(3, 3),
                ),
              ],
              items: [
                CollapsibleItem(
                  text: 'Teste',
                  icon: MfgLabs.user,
                  onPressed: () => {},
                  isSelected: true,
                ),
              ],
              body: Neumorphic(),
            ),
          ),
        ),
      ),
    );
  }
}
