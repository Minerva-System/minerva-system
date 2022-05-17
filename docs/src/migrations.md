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

Para começar, crie um contêiner Docker com o banco de dados:

```bash
./make_docker_db.sh
```

Isso criará um contêiner com PostgreSQL 14 chamado `minerva-micro`. Caso o
contêiner já exista, veja mais abaixo como removê-lo.

Após a criação do contêiner, o processo de preparação do PostgreSQL pode
ser um pouco demorado. Acompanhe este processo observando os logs:

```bash
docker logs -f minerva-micro
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

Para remover o banco de dados de teste criado no Docker, use os comandos a seguir.

Estes comandos servem para, respectivamente, parar a execução do contêiner e
então excluí-lo.

```bash
docker stop minerva-micro
docker rm minerva-micro
```

