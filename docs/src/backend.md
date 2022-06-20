# Back-End

O back-end Minerva compõe-se de microsserviços, com uma interface comum de
comunicação externa que seja simples de usar para os padrões atuais.

O back-end compõe-se dos seguintes componentes:

1. Componente de comunicação externa: um serviço composto de rotas, sendo
   portanto uma API REST. Este serviço requisita dados sob demanda a cada
   serviço, dependendo do recurso que foi requisitado por via externa. É
   efetivamente o intermediário entre Minerva e o mundo externo. As
   requisições entre este serviço e os outros deverão ser feito através da
   abertura de uma requisição gRPC em que este módulo seja o cliente
   requisitante; as respostas recebidas via gRPC são então retornadas
   como resposta às requisições recebidas via REST, após tratamento para
   serialização como JSON.
2. Componente de usuários: Servidor gRPC responsável por realizar o CRUD
   de usuários e por verificar as regras de negócio destas operações.
3. Componente de sessão: Servidor gRPC responsável por realizar login,
   logoff, verificação de senha e gerenciamento de sessão de usuários.
4. Componente de produtos: Servidor gRPC responsável por realizar o CRUD
   de produtos e por verificar as regras de negócio destas operações.
5. Componente de estoque: Servidor gRPC responsável por realizar regras
   de negócios relacionadas a estoque de produtos (início, baixa, lançamento,
   etc).

Os **serviços gRPC** supracitados tratam-se de servidores gRPC que podem
receber conexões vindas do ponto de entrada REST ou mesmo entre si. Além
disso, os serviços gRPC devem ser capazes de se comunicar com bancos de
dados, que são recursos essenciais para os mesmos (exemplo: PostgreSQL,
Redis). Além disso, **estes serviços devem gravar log de suas operações**,
mais especificamente nas operações de inserção, atualização e exclusão.

Esses componentes possuem análogos programados, mas não são todos os módulos
da aplicação, que também constituem-se de bibliotecas que podem ser utilizadas
e referenciadas entre si.

<style>
svg:not(:root ) {
      max-width: 100%;
	  height: auto;
}
</style>

<center>

```dot process
graph {
	bgcolor=transparent;
	rankdir="TB";
	compound=true;
	node[style=filled; fillcolor="#999999"];
	
	frontend[label="FRONT-END", shape=note, color=darkorange, fontcolor=darkorange, fillcolor=transparent];
	
	subgraph cluster_db {
		rankdir="LR";
		label = "BANCO DE DADOS\n(multi-tenant)";
		color=darkmagenta;
		fontcolor=darkmagenta;
		db2[label="inq3", shape=cylinder, color=darkmagenta, fontcolor=darkmagenta];
        db1[label="inq2", shape=cylinder, color=darkmagenta, fontcolor=darkmagenta];
		db3[label="inq1", shape=cylinder, color=darkmagenta, fontcolor=darkmagenta];
		{rank=same; db3; db1; db2;}
	}

	subgraph cluster_backend {
		label = "BACK-END";
		fontcolor=darkred;
		rankdir="LR";
		color=darkred;
		
		rest[label="REST", shape=box3d, color=green, fontcolor=green];
		
		tenancy[label="TENANCY", shape=box3d, color=blue, fontcolor=blue];
		user[label="USER", shape=box3d, color=blue, fontcolor=blue];
		session[label="SESSION", shape=box3d, color=blue, fontcolor=blue];
		product[label="PRODUCT", shape=box3d, color=blue, fontcolor=blue];
		stock[label="STOCK", shape=box3d, color=blue, fontcolor=blue];
		runonce[label="RUNONCE", shape=box, color=lightblue, fontcolor=blue];
		report[label="REPORT", shape=box3d, color=blue, fontcolor=blue];
		client[label="CLIENT", shape=box3d, color=blue, fontcolor=blue];
		audit[label="AUDIT", shape=box3d, color=blue, fontcolor=blue];
		comm[label="COMM", shape=box3d, color=blue, fontcolor=blue];
		
		user -- session [color=blue, fontcolor=blue, penwidth=2.0];
		comm -- client [color=blue, fontcolor=blue, penwidth=2.0];
		
		rest -- tenancy[color=blue, fontcolor=blue, penwidth=2.0];
		rest -- user[color=blue, fontcolor=blue, penwidth=2.0];
		rest -- session[color=blue, fontcolor=blue, penwidth=2.0];
		rest -- product[color=blue, fontcolor=blue, penwidth=2.0];
		rest -- stock[color=blue, fontcolor=blue, penwidth=2.0];
		rest -- client[color=blue, fontcolor=blue, penwidth=2.0];
		rest -- audit[color=blue, fontcolor=blue, penwidth=2.0];
		rest -- comm[color=blue, fontcolor=blue, penwidth=2.0];

		rest -- report[color=blue, fontcolor=blue, penwidth=2.0];
		
		tenancy -- db1 [lhead=cluster_db, color=darkmagenta, fontcolor=darkmagenta, penwidth=2.0];
		user -- db1 [lhead=cluster_db, color=darkmagenta, fontcolor=darkmagenta, penwidth=2.0];
		session -- db1 [lhead=cluster_db, color=darkmagenta, fontcolor=darkmagenta, penwidth=2.0];
		product -- db1 [lhead=cluster_db, color=darkmagenta, fontcolor=darkmagenta, penwidth=2.0];
		stock -- db1 [lhead=cluster_db, color=darkmagenta, fontcolor=darkmagenta, penwidth=2.0];
		runonce -- db1 [lhead=cluster_db, color=magenta, fontcolor=magenta, penwidth=2.0];
		client -- db1 [lhead=cluster_db, color=darkmagenta, fontcolor=darkmagenta, penwidth=2.0];
		audit -- db1 [lhead=cluster_db, color=darkmagenta, fontcolor=darkmagenta, penwidth=2.0];
	}

	frontend -- rest[color=green, fontcolor=green, penwidth=2.0];
	
}
```

</center>

```dot process
graph {
	bgcolor=transparent;
	rankdir="TB";
	compound=true;
	node[style=filled; fillcolor=transparent];
	
	subgraph cluster_legenda {
		label = "Legenda: Tipos de Conexão";
		style = filled;
		fillcolor = white;
		key [label=<
		  <table border="0" cellpadding="2" cellspacing="0" cellborder="0">
			<tr><td align="right" port="i1">gRPC  </td></tr>
            <tr><td align="right" port="i2">Banco de Dados via Pool  </td></tr>
            <tr><td align="right" port="i3">Banco de Dados Avulsa  </td></tr>
            <tr><td align="right" port="i4">HTTP via REST  </td></tr>
          </table>
		>; shape=plaintext]
        key2 [label=<
		  <table border="0" cellpadding="2" cellspacing="0" cellborder="0">
            <tr><td port="i1">&nbsp;</td></tr>
            <tr><td port="i2">&nbsp;</td></tr>
            <tr><td port="i3">&nbsp;</td></tr>
            <tr><td port="i4">&nbsp;</td></tr>
          </table>
	    >; shape=plaintext]
		{rank=same; rankdir=LR; key; key2; }
		key:i1:e -- key2:i1:w [color=blue; penwidth=2.0];
		key:i2:e -- key2:i2:w [color=darkmagenta; penwidth=2.0];
        key:i3:e -- key2:i3:w [color=magenta; penwidth=2.0];
        key:i4:e -- key2:i4:w [color=green; penwidth=2.0];
  }
}
```

## Bibliotecas

As bibliotecas planejadas para o sistema são:

- [x] `minerva-rpc`: Implementação de protocolos gRPC e de mensagens destes
   protocolo. Deve ser importado em todos os módulos, sendo essencial para
   a criação de clientes e servidores gRPC. Os modelos de comunicação
   implementados para si devem ser também convertidos para e
  a partir dos DTOs do módulo de dados.
- [x] `minerva-data`: Implementação de utilitários de comunicação com banco de
  dados (PostgreSQL) e objetos de transferência de dados (DTOs). Deve ser
  importado em todos os módulos, exceto na comunicação REST. Os DTOs também
  devem implementar traits e utilitários para conversão das mensagens
  implementadas em `minerva-rpc` para os DTOs desta biblioteca.
- [ ] `minerva-cache`: Implementação de utilitários de comunicação com
  cache, message brokers e armazenamento temporário _in-memory_ (Redis).
  Deve ser importado principalmente no módulo de sessão.

## Módulos

Os módulos planejados para o sistema são:

- [ ] `minerva-tenancy`: Servidor gRPC para CRUD de inquilinos. Deve ser
  capaz de gerenciar inquilinos, mas um inquilino não pode ser deletado
  através desse serviço, apenas desativado. Apenas administradores do sistema
  podem ter acesso.
- [x] `minerva-user`: Servidor gRPC para CRUD de usuários. Deve ser capaz de
  manipular as regras de negócios relacionadas a clientes.
- [x] `minerva-session`: Servidor gRPC para gerência de sessão de usuário.
- [ ] `minerva-product`: Servidor gRPC para CRUD de produtos. Deve ser capaz
  de manipular as regras de negócios relacionadas a produtos, mas que não
  envolvam controle de estoque.
- [ ] `minerva-stock`: Servidor gRPC para CRUD de estoque de produtos. Deve
  ser capaz de manipular as regras de negócios relacionadas a estoque, mas
  que não envolvam manipulação de produtos.
- [x] `minerva-rest`: Servidor REST para comunicação com os demais módulos
  executáveis. Possui rotas que apontam para serviços específicos, e é por
  definição um cliente gRPC de todos os servidores gRPC.
- [x] `minerva-runonce`: Serviço **avulso** para configuração do ambiente, de
  forma assíncrona. Responsável pela execução de migrações do banco de dados
  e outras operações de configuração inicial.
- [ ] `minerva-report`: Servidor gRPC para geração de relatórios. Deve receber
  dados com formatação esperada de um relatório, e então deverá gerar um
  arquivo PDF e retorná-lo inteiramente como resposta.
- [ ] `minerva-client`: Servidor gRPC para CRUD de clientes. Deve ser capaz
  de manipular as regras de negócios relacionadas a clientes.
- [ ] `minerva-audit`: Servidor gRPC para gerenciamento de logs de auditoria.
  Possibilita a consulta aos logs de auditoria do sistema.
- [ ] `minerva-comm`: Servidor gRPC para comunicação externa com clientes
  via mensagens instantâneas.

## Portas

Os serviços, independente de serem gRPC ou REST, devem ser executados em
certas portas padrão para evitarem conflitos durante o tempo de depuração.
Cada porta deve também ser configurável através de variáveis de ambiente.

A tabela a seguir discrimina as variáveis de ambiente e as portas padrão
de acordo com o serviço em questão.

| Serviço | Variável               | Valor |
|---------|------------------------|-------|
| REST    | `ROCKET_PORT`          | 9000  |
| USER    | `USER_SERVICE_PORT`    | 9010  |
| SESSION | `SESSION_SERVICE_PORT` | 9011  |
| PRODUCT | `PRODUCT_SERVICE_PORT` | 9012  |
| STOCK   | `STOCK_SERVICE_PORT`   | 9013  |
| REPORT  | `REPORT_SERVICE_PORT`  | 9014  |
| TENANCY | `TENANCY_SERVICE_PORT` | 9015  |
| CLIENT  | `CLIENT_SERVICE_PORT`  | 9016  |
| AUDIT   | `AUDIT_SERVICE_PORT`   | 9017  |
| COMM    | `COMM_SERVICE_PORT`    | 9018  |


No caso do serviço REST, verifique o arquivo `Rocket.toml` para avaliar
a configuração em desenvolvimento e em produção do mesmo.

## Gateways

Os serviços também podem operar em máquinas diferentes, dependendo de sua
rota.

Normalmente, quando todos os serviços são executados manualmente na mesma
máquina, operamos com uma rota `localhost`. Nesse caso, a variável de
ambiente de cada serviço é definida como esse valor.

Todavia, num ambiente de orquestração de contêineres (como Docker Compose
ou Kubernetes), cada serviço estará operando de forma separada, e poderá
comunicar-se com os outros serviços por intermédio de uma rede interna
ao qual apenas os serviços têm acesso de forma explícita. Assim, as
variáveis de ambiente que determinam o nome do servidor devem ser definidas
manualmente, de acordo com a forma como o deploy de cada serviço foi
realizado.

A seguir, temos uma tabela relacionando variáveis de ambiente com seus
devidos valores, que serão resolvidos através do DNS da rede interna criada
pelo orquestrador de contêineres.

No caso do serviço REST, verifique o arquivo `Rocket.toml` para avaliar
a configuração em desenvolvimento e em produção do mesmo.

| Serviço                       | Variável                  | Valor em Produção |
|-------------------------------|---------------------------|-------------------|
| USER                          | `USER_SERVICE_SERVER`     | `user`            |
| Banco de Dados Relacional     | `DATABASE_SERVICE_SERVER` | `postgresql`      |
| Banco de Dados Não-Relacional | `MONGO_SERVICE_SERVER`    | `mongodb`         |
| REST                          | nenhuma                   | `rest`            |
| RUNONCE                       | nenhuma                   | `runonce`         |
| SESSION                       | `SESSION_SERVICE_SERVER`  | `session`         |

