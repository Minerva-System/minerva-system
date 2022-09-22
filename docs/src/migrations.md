# Executando migrations

As migrations são uma parte vital do Sistema Minerva, não apenas porque
definem as tabelas do banco de um *tenant*, mas porque também definem os
*schemas* para a programação dos módulos.




## Pré-requisitos

- [Rust](https://rustup.rs) (compilador `rustc` e gerenciador de
  pacotes `cargo`, versão 1.60.0 ou superior);
- [Diesel](https://diesel.rs) (versão 1.4.1 ou superior, com suporte
  a PostgreSQL);
- `diesel_cli` com suporte a PostgreSQL;
- Docker versão 20.10 ou superior.

Para instalar o `diesel_cli` apenas com suporte a PostgreSQL, use o
seguinte comando:

```bash
cargo install diesel_cli --no-default-feature --features postgres
```




## Considerações importantes

**Toda e qualquer migration deve ser criada no diretório do módulo
`minerva-runonce`**, especificamente porque este diretório possui também
as configurações de acesso e de geração de *schema* em
`minerva-data/src/schema.rs`.

Além disso, **sempre execute todos os comandos abaixo no diretório do módulo
`minerva-runonce`**.




## Configuração inicial

Para começar, crie um contêiner Docker para cada um dos bancos de dados:

```bash
./make_postgres_db.sh
./make_mongo_db.sh
./make_redis_db.sh
```

Isso criará contêineres executando PostgreSQL 14, MongoDB 6 e Redis 7.

Após a criação dos contêineres, o processo de preparação dos bancos de dados
pode ser um pouco demorado. Acompanhe este processo observando os logs:

```bash
# Banco relacional
docker logs -f minerva-postgres

# Banco não-relacional
docker logs -f minerva-mongo

# Cache
docker logs -f minerva-redis
```

Em seguida, execute a operação inicial de criação de um banco de dados.
Para tanto, vamos criar um banco chamado `minerva` e executar todas as
migrations nele, logo de cara:

```bash
diesel setup --database-url="postgres://postgres:postgres@localhost/minerva"
```




## Criando uma migration

Para criar uma *migration*, use um comando similar ao seguinte:

```bash
diesel migration generate <nome_da_migration>
```

Substitua `<nome_da_migration>` por um nome que faça sentido.
Isso gerará uma nova *migration* no diretório `migrations`,
que possuirá os arquivos `up.sql` e `down.sql`. Edite-os de acordo
com o necessário.




## Executando migrations

Para executar todas as *migrations* pendentes, execute o comando:

```bash
diesel migration run --database-url="postgres://postgres:postgres@localhost/minerva"
```

Isso também poderá reconstruir o arquivo `minerva-data/src/schema.rs`, a depender
de mudanças no *schema*.

Para testar a última migration executada:

```bash
diesel migration redo --database-url="postgres://postgres:postgres@localhost/minerva"
```




## Removendo banco de dados de teste

Para remover os bancos de dados de teste criados no Docker, use os comandos a
seguir.

Estes comandos servem para, respectivamente, parar a execução dos contêineres e
então excluí-los.

```bash
docker stop minerva-postgres minerva-mongo minerva-redis
docker rm minerva-postgres minerva-mongo minerva-redis
```

