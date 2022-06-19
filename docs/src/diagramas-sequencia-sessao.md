# Diagramas de Sequência: Sessão

Os diagramas a seguir dizem respeito ao fluxo de gerenciamento da sessão de um
usuário do sistema.

A gerência da sessão assume que os dados do usuário possam ou não serem
encontrados no banco de dados. Dito isso, não é incumbência da sessão realizar
operações CRUD como usuários, e sim com a entidade da sessão em si.
