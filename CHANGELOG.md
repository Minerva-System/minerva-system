# Changelog

Todas as mudanças notáveis neste projeto serão documentadas nesse arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Versionamento Semântico](https://semver.org/spec/v2.0.0.html).



## [Não-lançado]


Relação de versões de microsserviços:

- `USER` - v0.2.2
- `SESSION` - v0.1.2
- `RUNONCE` - v0.2.1
- `REST` - v0.2.2
- Front-End - v0.1.1 (pré-alfa)

### Adicionado

- *`SESSION`:* Alteração do serviço para abrigar uso de _cache_ via Redis;
- *`REST`:* _Catchers_ para tipos de retorno comuns e retorno genérico padrão;
- *Documentação:* Adição de diagramas iniciais de caso de uso e sequência;
- *Projeto:* Adição de CHANGELOG e regras de versionamento semântico.

### Modificado

- *`USER`:* Alteração do nome do serviço de `USERS` para `USER`, evitando
  maiores enganos;
- *Geração de Imagens:* Dockerfile para gerar imagens agora foi unificado,
  incluindo compilação do frontend também no script, e agora utiliza
  BuildKit por padrão;
- *Geração de Imagens:* Imagens Docker agora são geradas usando Alpine Linux
  como base, reduzindo tamanho e _footprint_ em _deploys_ no Compose/Swarm/K8s.

### Consertado

- *`REST`:* Erros na conexão com um microsserviço agora retornam um erro 503
  (Recurso Indisponível);
- *Documentação:* Problema na exportação de diagramas usando PlantUML no Github
  Pages.

### Removido

- *Documentação:* Removido o Dockerfile específico para PgAdmin4. A partir de
  agora, será usada a imagem oficial do PgAdmin4, e o arquivo de configuração
  será montado como necessário (via arquivos de configuração do Docker Compose
  e do Docker Stack, ou via ConfigMap no K8s).

### Segurança

- *Geral:* Removida a dependência da _crate_ `rustc-serialize` na configuração
  da _crate_ `chrono` (em confirmidade com alerta Dependabot), para todos os
  módulos.

### Problemas conhecidos

- *Geração de Imagens:* O _target_ para ARM64 na criação das imagens Docker foi
  desabilitado até que seja corrigido
  [um bug no BuildKit](https://github.com/docker/build-push-action/issues/621)
  que faz com que o Qemu consuma RAM arbitrariamente ao realizar compilação
  via emulação de hardware.


## [v1] - 2022-06-17

Relação de versões de microsserviços:

- `USERS` - v0.2.1
- `SESSION` - v0.1.1
- `RUNONCE` - v0.2.0
- `REST` - v0.2.0
- Front-End - v0.0.1 (pré-alfa)

### Adicionado

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
<!-- ## [v1] - 20XX-XX-XX -->
<!-- ### Adicionado -->
<!-- ### Modificado -->
<!-- ### Consertado -->
<!-- ### Removido -->
<!-- ### Segurança  -->
<!-- ### Problemas conhecidos  -->
