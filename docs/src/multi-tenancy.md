# Multi-Tenancy

O Minerva System é um sistema multi-tenancy. Isso significa que é
capaz de gerenciar bancos de dados diferentes dependendo do tenant
(cliente do serviço) atual. No Minerva System, isso é gerenciado de
acordo com a forma como as requisições são recebidas.

## Configuração

Os tenants devem ser gerenciados através do arquivo `tenancy.toml`.

A seguir, um exemplo do conteúdo em potencial deste arquivo.

```toml
[[tenants]]
name = "Minerva System"
database = "minerva"
connections = 5

[[tenants]]
name = "Test Database"
database = "teste"
connections = 5

[[tenants]]
name = "Comercial Fulano S/A"
database = "comercial-fulano"
connections = 5
```

## Criação do banco de dados

O serviço `RUNONCE` deverá executar a criação do banco de dados, caso
não seja possível conectar-se ao mesmo. Isso deve ser feito sobretudo
através da leitura do arquivo `tenancy.toml`, encontrado na pasta de
execução do projeto.

Caso um novo tenant seja adicionado ao sistema, o serviço `RUNONCE`
deverá ser forçadamente executado para que o sistema fique apto a
utilizar o banco de dados para aquele tenant.

O sistema `RUNONCE` deverá, para cada tenant listado em `tenancy.toml`:

1. Tentar conectar-se ao banco em questão. Se isso não for possível,
   deverá criá-lo;
2. Executar as migrations para aquele banco;
3. Criar o usuário `admin` para aquele banco.

