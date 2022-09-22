# Compilando e executando com recursos da máquina

Você pode compilar os módulos do sistema individualmente e executá-los
usando o próprio ambiente Rust.

## Introdução

Este artigo trata da situação mais comum durante o desenvolvimento das
partes do sistema, que envolve utilizá-las individualmente em um
ambiente de desenvolvimento.

O *deploy* usando Docker Compose e Kubernetes, enquanto possível em
ambiente de homologação e de testes manuais, utiliza muitos recursos
da máquina, e não é o ideal de ser utilizado enquanto o programador
estiver debugando a aplicação. Além disso, pela própria forma como
o sistema foi planejado, é possível executar porções individuais do
sistema em que haja interdependência entre elas.




## Objetivo

Compilar todos os módulos ou módulos individuais é muito importante do
ponto de vista do desenvolvimento. Neste artigo, tratamos de como isso
pode ser feito na máquina local de um desenvolvedor.




## Dependências

Você precisará de:

- [Rust](https://rustup.rs) (compilador `rustc` e gerenciador de
  pacotes `cargo`, versão 1.60.0 ou superior);
- [Diesel](https://diesel.rs) (versão 1.4.1 ou superior, com suporte
  a PostgreSQL);
- [Flutter](https://flutter.dev) (versão 3.0.0 ou superior, canal
  `stable`. Apenas necessário o *target* de compilação para `web`);
- Dart (versão 2.17.0 ou superior, canal `stable`);
- Docker (versão 2.10 ou superior).

O compilador Rust e o Docker são essenciais para compilar os módulos
individuais do *back-end* do projeto, enquanto o Flutter é importante
para a confecção do *front-end* da aplicação. Sendo assim, as
dependências podem ser instaladas de acordo com o bom-senso do
desenvolvedor.

O Diesel pode ser instalado através do gerenciador de pacotes `cargo`,
e sua instalação pode ser consultada em seu site, linkado acima. Além
disso, a linguagem Dart será instalada através do Flutter, de acordo
com as instruções que podem ser encontradas no site do mesmo.





## Estrutura do projeto

O repositório do projeto é um *monorepo*, isto é, engloba todas as
partes do sistema inteiro. Por isso, as partes relacionadas a *back-end*
estão dispostas em um *Workspace*, configurável através das próprias
ferramentas do `cargo` e da linguagem Rust, enquanto o *front-end*
encontra-se unicamente no diretório `minerva_frontend`, e não faz
parte do *Workspace* em si.





## Preparação do ambiente

A primeira parte a ser executada deverá ser a preparação do ambiente.
Isso inclui a preparação de quaisquer serviços ou bancos de dados
externos que possam ser importantes para a execução da aplicação.

No Sistema Minerva, o serviço `RUNONCE` é responsável por executar
essas operações, sendo também o serviço que executa *migrations* no
banco de dados, por exemplo.

Para tanto, precisaremos compilar este módulo específico antes de
qualquer outro. Isso será melhor delineado na seção sobre compilação
do *back-end*, mas realizaremos uma configuração rápida nesta seção.



### Criando recursos locais com Docker

Para executar localmente, o sistema precisa que alguns serviços sejam
instanciados antes de sua execução.

É importante lembrar que os métodos a seguir não são considerados
seguros para persistência de dados. Por isso, use-os apenas com a
finalidade de testes.

#### Banco de dados relacional

Como primeira dependência, recomenda-se criar o banco de dados relacional
via Docker. Também seria possível instalar o PostgreSQL 14 na máquina local,
mas o Docker provê a comodidade necessária para o BD.

Para subir o banco de dados relacional, execute o script `make_postgres_db.sh`
no diretório `minerva-runonce`, ou execute:

```bash
docker run --name minerva-postgres \
       -e POSTGRES_USER=postgres \
       -e POSTGRES_PASSWORD=postgres \
       -p 5432:5432 \
       -d postgres:14-alpine
```

O contêiner poderá então ser gerenciado através do Docker, como um contêiner
qualquer.

#### Banco de dados não-relacional

Recomenda-se também criar o banco de dados não-relacional via Docker.
Igualmente, é possível instalar o MongoDB 6 por métodos convencionais.

Para subir o banco de dados não-relacional, execute o script
`make_mongo_db.sh` no diretório `minerva-runonce`, ou execute:

```bash
docker run --name minerva-mongo \
       -e MONGO_INITDB_ROOT_USERNAME=root \
       -e MONGO_INITDB_ROOT_PASSWORD=mongo \
       -p 27017:27017 \
       -d mongo:6
```

Como esperado, o contêiner pode ser gerenciado normalmente através do Docker.

#### Banco de dados _in-memory_ (cache)

Para o uso do serviço de cache, recomenda-se usar diretamente o Redis através
do Docker. Para tanto, execute o script `make_redis_db.sh` no diretório
`minerva-runonce`, ou execute:

```bash
docker run --name minerva-redis \
       -p 6379:6379 \
       -d redis:7-alpine
```

Assim como antes, gerencie o contêiner criado usando o Docker.

#### Broker de mensagens (com gerenciador)

O _message broker_ RabbitMQ deverá também ser executado através do Docker.
Para a execução do mesmo, todavia, atente-se para o uso de memória elevado
que essa ferramenta possui.

Você poderá executar o script `make_rabbitmq.sh` no diretório
`minerva_runonce`, ou simplesmente executar:

```bash
docker run --name minerva-rabbitmq \
       -e RABBITMQ_DEFAULT_USER=rabbitmq \
       -e RABBITMQ_DEFAULT_PASS=minerva \
       -p 15672:15672 \
       -p 5672:5672 \
       -d rabbitmq:3-management-alpine
```

### Executando configuração inicial (módulo `RUNONCE`)

A seguir, execute o módulo `RUNONCE` para preparar todos os bancos de
dados de *tenants*, executar as migrations, criar o usuário `admin`
em cada banco e outras operações em outros serviços também.

Você poderá executar o módulo diretamente a partir da raiz do projeto:

```bash
cargo run --bin minerva-runonce
```

Caso haja algum problema com o comando anterior (por exemplo, se o
módulo não encontrar o diretório `migrations`), vá para o diretório do
módulo e execute-o:

```bash
cd minerva-runonce
cargo run
```

Após a compilação do módulo `RUNONCE`, o mesmo aguardará o banco de
dados estar pronto para receber as conexões e aplicará as migrations.




## Compilação (*Back-end*)

Você poderá compilar todos os módulos do projeto de uma só vez, ou
compilar apenas os módulos necessários.



### Compilando todos os módulos

Para compilar todos os módulos, vá para a raiz do projeto e execute
um comando de compilação para todo o workspace:

```bash
cargo build
```

De forma similar, você poderá compilar o projeto para produção através
da *flag* `--release`:

```bash
cargo build --release
```


### Compilando um módulo específico

Existem duas formas de compilar um módulo específico: a partir do
*workspace* (diretório raiz do repositório) ou a partir do diretório
do módulo específico.

Qualquer módulo pode ser compilado a partir do diretório raiz com um
comando como o mostrado a seguir (substitua `<módulo>` pelo nome do
diretório do módulo em questão):

```bash
cargo build --bin <módulo>
```

Isto compilará qualquer módulo que faça parte do *workspace*, exceto
bibliotecas auxiliares (como `minerva-rpc`, `minerva-data` e
`minerva-cache`) e o *front-end* (contido em `minerva_frontend`).

Da mesma forma, você também poderá ir ao diretório do módulo específico
e compilá-lo diretamente; neste caso, a compilação também funcionará
para bibliotecas auxiliares.

```bash
cd <módulo>
cargo build
```

De forma similar à compilação geral, ambos os comandos também admitem
a *flag* `--release` para compilar os módulos para produção.



### Execução

É possível executar diretamente um módulo qualquer através do `cargo`,
o que também implica na sua compilação.

Para executar a partir do diretório do *workspace* (apenas para
módulos que geram executáveis):

```bash
cargo run --bin <módulo>
```

Para executar a partir do diretório do módulo em questão:

```bash
cd <módulo>
cargo run
```

Da mesma forma, é possível compilar e executar os módulos no modo
de produção através da *flag* `--release`.



### Testes

Para executar testes unitários e integração, basta seguir um processo
similar à execução dos módulos. Testes com binários compilados para
produção podem ser igualmente controlados pela *flag* `--release`.

Para executar quaisquer testes, é necessário **garantir** que o
banco de dados esteja acessível e adequadamente configurado.

```bash
# Para testar todos os módulos do workspace
cargo test

# Para testar apenas um módulo do workspace
cargo test --bin <módulo>

# Para testar apenas um módulo em seu diretório
cd <módulo>
cargo test
```



## Compilação (*Front-end*)

O front-end é um módulo separado do restante dos módulos, sendo o
sistema que envolve a interface gráfica do Sistema Minerva.




### Executando o projeto via console

Para executar o projeto via console, basta usar a ferramenta de linha
de comando do Flutter.


#### Preparando o Flutter

Antes de mais nada, garanta que o Flutter esteja configurado para
compilar projetos Web:

```bash
flutter config --enable-web
```

Além disso, o Google Chrome deverá estar disponível para ser utilizado
no debug. O estado do ambiente Flutter pode ser verificado com o
comando `flutter doctor`.

Caso haja alguma inconsistência no seu ambiente, veja a seção de
[preparação do Flutter para Web](https://docs.flutter.dev/get-started/web)
na documentação oficial.


#### Executando o projeto

Para executar o projeto, vá até o diretório do módulo de *front-end*,
baixe as dependências necessárias, e então execute o projeto no Google
Chrome:

```bash
cd minerva_frontend
flutter pub get
flutter run -d chrome
```


### Compilando para produção

Para compilar o projeto para produção, vá até a pasta do módulo e
execute os comandos a seguir. Eles baixarão as dependências faltantes
(caso já não tenham sido baixadas) e gerarão os arquivos estáticos
da aplicação.

```bash
cd minerva_frontend
flutter pub get
flutter build web
```

Você poderá encontrar a versão compilada da aplicação *front-end* no
diretório `minerva_frontend/build/web`.

