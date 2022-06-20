# Changelog

Todas as mudanças notáveis neste projeto serão documentadas nesse arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Versionamento Semântico](https://semver.org/spec/v2.0.0.html).



## [Não-lançado]

### Geral

Relação de versões de microsserviços:

- `USERS` - v0.2.1
- `SESSION` - v0.1.1
- `RUNONCE` - v0.2.1
- `REST` - v0.2.1
- Front-End - v0.1.1 (pré-alfa)

#### Adicionado

- Adição de diagramas iniciais de caso de uso e sequência;
- Adição de CHANGELOG e regras de versionamento semântico.

#### Consertado

- Problema na exportação de diagramas usando PlantUML no Github Pages.

#### Removido

- Dependência da _crate_ `rustc-serialize` na configuração da _crate_
  `chrono` (em confirmidade com alerta Dependabot), para todos os módulos.

### `REST` - v0.2.1

#### Adicionado

- _Catchers_ para tipos de retorno comuns e retorno genérico padrão.

#### Consertado

- Erros na conexão com um microsserviço agora retornam um erro 503 (Recurso
  Indisponível).




## [v1] - 2022-06-17

### Geral

Relação de versões de microsserviços:

- `USERS` - v0.2.1
- `SESSION` - v0.1.1
- `RUNONCE` - v0.2.0
- `REST` - v0.2.0
- Front-End - v0.0.1 (pré-alfa)

#### Adicionado

- Criação de schemas do banco de dados relacional (PostgreSQL 14);
- Criação das coleções do banco de dados não-relacional (MongoDB 5);
- Adição de _protocol buffers_;
- Adição do microsserviço gRPC `USERS`;
- Adição do microsserviço gRPC `SESSION`;
- Adição da base para alguns outros microsserviços;
- Adição da documentação básica;
- Adição das bibliotecas `DATA` e `RPC`;
- Adição do microsserviço gRPC `REST` (Rocket v0.5.0-rc.2);
- Adição de rotas de autenticação e CRUD de usuários;
- Adição de _pooling_ de conexões com o banco de dados não-relacional;
- Adição de _logs_ para operações de CRUD de usuários e de sessão;
- Adição de configuração de teste para Docker Compose;
- Adição de configuração de deploy para Docker Swarm;
- Adição de configuração de deploy para Kubernetes;
- Adição de conceito básico de Front-End (com Flutter 3.0);
- Adição de automatização de testes;
- Adição de geração de documentação (mdBook, `cargo doc`, `flutter doc`)
  via GitHub Pages.


[Não-lançado]: https://github.com/luksamuk/minerva-system/compare/v1...HEAD
[v1]: https://github.com/luksamuk/minerva-system/releases/tag/v1

<!-- ==== Exemplo ==== -->
<!-- ## [v1] - 2022-06-17 -->
<!-- #### Adicionado -->
<!-- #### Modificado -->
<!-- #### Consertado -->
<!-- #### Removido -->

<!-- ### `USERS` - [v0.2.1] -->
<!-- #### Adicionado -->
<!-- #### Modificado -->
<!-- #### Consertado -->
<!-- #### Removido -->

<!-- ### `SESSION` - [v0.1.1] -->
<!-- #### Adicionado -->
<!-- #### Modificado -->
<!-- #### Consertado -->
<!-- #### Removido -->

<!-- ### `RUNONCE` - [v0.2.0] -->
<!-- #### Adicionado -->
<!-- #### Modificado -->
<!-- #### Consertado -->
<!-- #### Removido -->

<!-- ### `REST` - [v0.2.0] -->
<!-- #### Adicionado -->
<!-- #### Modificado -->
<!-- #### Consertado -->
<!-- #### Removido -->

<!-- ### Front-End - [v0.0.1] - (pré-alfa) -->
<!-- #### Adicionado -->
<!-- #### Modificado -->
<!-- #### Consertado -->
<!-- #### Removido -->
