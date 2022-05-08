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

```dot process
graph {
	rankdir="TB";
	compound=true;
	node[style=filled; fillcolor=white];
	
	frontend[label="FRONT-END", shape=note, color=darkorange, fontcolor=darkorange];
	
	subgraph cluster_db {
		rankdir="LR";
		label = "BANCO DE DADOS\n(multi-inquilino)";
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
		
		rest[label="REST", shape=box3d];
		
		user[label="USER", shape=box3d];
		session[label="SESSION", shape=box3d];
		product[label="PRODUCT", shape=box3d];
		stock[label="STOCK", shape=box3d];
		runonce[label="RUNONCE", shape=box3d];
		report[label="REPORT", shape=box3d];
		
		{rank="same"; rest; report;}
		{rank="same"; user; session; product; stock; runonce;}
		
		rest -- user[label="gRPC", color=blue, fontcolor=blue];
		rest -- session[label="gRPC", color=blue, fontcolor=blue];
		rest -- product[label="gRPC", color=blue, fontcolor=blue];
		rest -- stock[label="gRPC", color=blue, fontcolor=blue];
		
		user -- session[label="gRPC", color=blue, fontcolor=blue];
		product -- stock[label="gRPC", color=blue, fontcolor=blue];
		rest -- report[label="gRPC", color=blue, fontcolor=blue];
		
		user -- db1 [lhead=cluster_db, label="DB\n(Pool)", color=darkmagenta, fontcolor=darkmagenta];
		session -- db1 [lhead=cluster_db, label="DB\n(Pool)", color=darkmagenta, fontcolor=darkmagenta];
		product -- db1 [lhead=cluster_db, label="DB\n(Pool)", color=darkmagenta, fontcolor=darkmagenta];
		stock -- db1 [lhead=cluster_db, label="DB\n(Pool)", color=darkmagenta, fontcolor=darkmagenta];
		runonce -- db1 [lhead=cluster_db, label="DB\n(Avulsa)", color=magenta, fontcolor=magenta];
	}

	frontend -- rest[label="REST", color=green, fontcolor=green];
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

- [x] `minerva-user`: Servidor gRPC para CRUD de usuários. Deve ser capaz de
  manipular as regras de negócios relacionadas a clientes.
- [ ] `minerva-session`: Servidor gRPC para gerência de sessão de usuário.
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

## Portas

Os serviços, independente de serem gRPC ou REST, devem ser executados em
certas portas padrão para evitarem conflitos durante o tempo de depuração.
Cada porta deve também ser configurável através de variáveis de ambiente.

A tabela a seguir discrimina as variáveis de ambiente e as portas padrão
de acordo com o serviço em questão.

| Serviço | Variável               | Valor |
|---------|------------------------|-------|
| USER    | `USER_SERVICE_PORT`    | 9010  |
| SESSION | `SESSION_SERVICE_PORT` | 9011  |
| PRODUCT | `PRODUCT_SERVICE_PORT` | 9012  |
| STOCK   | `STOCK_SERVICE_PORT`   | 9013  |
| REPORT  | `REPORT_SERVICE_PORT`  | 9014  |
| REST    | `ROCKET_PORT`          | 9000  |


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

| Serviço        | Variável                  | Valor em Produção |
|----------------|---------------------------|-------------------|
| USER           | `USER_SERVICE_SERVER`     | `users`           |
| Banco de Dados | `DATABASE_SERVICE_SERVER` | `postgresql`      |
| REST           | nenhuma                   | `rest`            |
| RUNONCE        | nenhuma                   | `runonce`         |

