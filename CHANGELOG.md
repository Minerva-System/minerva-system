# Changelog

Todas as mudanças notáveis neste projeto serão documentadas nesse arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Versionamento Semântico](https://semver.org/spec/v2.0.0.html).

## [Não-lançado]

Relação de versões de microsserviços:

- `USER` - v0.3.0
- `SESSION` - v0.1.2
- `RUNONCE` - v0.3.0
- `REST` - v0.3.1
- `DISPATCH`- v0.1.0
- Front-End - v0.1.1 (pré-alfa)

### Adicionado

- *Geração de imagens:* Reabilitado target de compilação para ARM64.
- *Geração de imagens:* Adicionado script que gera uma working tree limpa ao
  gerar recipe do Cargo Chef.
- *`RUNONCE`:* Estruturas para preparação de message broker (virtual hosts e filas
  fixas).
- *`DISPATCH`:* Criação do microsserviço.
- *Devcontainers do VSCode:* Adicionada configuração inicial (ainda instável).
- *Kubernetes:* Exposição dos serviços e ferramentas `REST`, Grafana e PgAdmin4
  nas rotas `/api`, `/grafana` e `/pgadmin`, respectivamente, através do uso
  de Ingresses.
- *Kubernetes:* Adicionados serviços Prometheus e Grafana, com alguns dashboards
  padronizados.
- *Minerva9:* Adicionada documentação para o MVC e repositório do projeto.
- *Kubernetes:* Adição de configurações para deploy em ambientes IoT.
- *`DATA`*: Adicionado DTO fixo para retorno de dados de sessão durante login.
- *REST:* API agora possui documentação e ferramentas para teste através de Swagger
  e RapiDoc (respectivamente através das rotas `/swagger` e `/rapidoc`).

### Modificado

- Rust: Define versão 1.65.0 para todo o projeto.
- *`RUNONCE`:* Spinlocks de aguardo de disponibilidade de serviços agora realizam
  _sleep_ assíncrono de dois segundos após cada falha.
- *`RUNONCE`:* Spinlocks de disponibilidade agora também operam de forma assíncrona.
- *Kubernetes:* Ajustes nos Ingresses existentes para que funcionem adequadamente,
  através de Traefik.
- *Kubernetes:* Ajustes nas configurações de deploy para que haja menos arquivos.
- *Kubernetes:* Ajustes nos limites de réplicas e de recursos requisitados para alguns
  serviços.
- *MongoDB:* Downgrade para versão 4.
- Tonic atualizado para v0.8.2.
- Prost atualizado para v0.11.
- *CI/CD:* Adicionada dependência do compilador de Protocol Buffers (`protoc`),
  no _build_ via Docker e no ambiente de testes do GitHub Actions.
- *Compose/Swarm:* Exposição das portas do PostgreSQL e do MongoDB para acesso remoto.
- *Compose/Swarm:* Removidas as ferramentas Mongo Express, Redis Commander e
  pgAdmin4, já que essas configurações são pensadas primariamente como debug.
  Para monitorar e inspecionar o MongoDB, o Redis e o PostgreSQL, veja ferramentas
  ad-hoc como MongoDB Compass, RESP.app ou DBeaver, respectivamente.
- *PostgreSQL:* Atualizado para versão 15.
- *Documentação:* Adicionados diagramas separados para cada _deployment_ no Kubernetes,
  bem como um diagrama geral da arquitetura do mesmo.
- *`REST`:* Requisições agora exigem token através de Bearer Token.
- *`REST`:* Requisições agora demandam _tenant_ no início das rotas.
- *`REST`:* Todas as rotas agora possuem tipos de resposta bem-definidos, possibilitando
  extração de schema para OpenAPI.
- *`REST`:* Alteradas as variáveis de ambiente designando profile e nível de log do
  Rocket nos vários tipos de deploy (local, Compose, Swarm, K8s)

### Consertado

- *`USER`:* Ao remover um usuário, envia mensagem requisitando remoção das sessões
  do mesmo (cache e coleção de sessões).
- *`USER`:* Caso um usuário falhe em ser criado (ao final da inserção), será considerado
  como se já existisse.
- *`REST` (K8s):* ConfigMap próprio estava sendo ignorado e agora é utilizado.
- *`REST`:* A API agora é capaz de lidar com a exposição de suas próprias rotas
  sob um endpoint específico (como `/api`, por exemplo; isso também funciona para a
  especificação OpenAPI e para Swagger e Rapidoc).

### Removido

- Removidos projetos Rust de módulos ainda não-iniciados, que causavam lentidão
  desnecessária na compilação.
- *`REST`:* Removidos exemplos de requisições na documentação das rotas (prefira a
  documentação via Postman ou use Swagger ou RapiDoc).
- *`REST`:* Removidas cores no texto do console durante deploy em k8s.

### Segurança

- Chaves dos dados de sessão armazenados no Redis agora são codificados usando
  Base64 para reduzir legibilidade.

### Problemas conhecidos

- A ferramenta *Redis Commander* conhecidamente funciona apenas em arquitetura
  AMD64, o que inviabiliza seu deploy no Kubernetes em ambientes ARM. Isso
  significa que clusters com o Minerva que sejam totalmente configurados em
  arquitetura ARM perderão alguma observabilidade quanto ao Redis. Adicionalmente,
  essa limitação também foi refletida na configuração da ferramenta via k8s.
- As configurações atuais para os Ingresses estão muito relacionadas ao que é
  necessário para realizar deploy em K3s, com Traefik sendo utilizado como
  backend para Ingresses. Isso inviabiliza um pouco o uso de Minikube e
  Microk8s.
- Builds de imagens Docker com Rust quebram o Qemu com uso excessivo de memória,
  caso a variável de ambiente `CARGO_NET_GIT_FETCH_WITH_CLI` não esteja definida
  como `true`.


## [v2] - 2022-06-05


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


[Não-lançado]: https://github.com/luksamuk/minerva-system/compare/v2...HEAD
[v2]: https://github.com/luksamuk/minerva-system/releases/tag/v2
[v1]: https://github.com/luksamuk/minerva-system/releases/tag/v1

<!-- ==== Exemplo ==== -->
<!-- ## [v1] - 20XX-XX-XX -->
<!-- ### Adicionado -->
<!-- ### Modificado -->
<!-- ### Consertado -->
<!-- ### Removido -->
<!-- ### Segurança  -->
<!-- ### Problemas conhecidos  -->
