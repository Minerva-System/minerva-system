# Sumário

- [Introdução](./intro.md)
- [Changelog](./CHANGELOG.md)
- [Estrutura geral do projeto](./estrutura.md)
  - [Front-End](./frontend.md)
	- [Interface para Plan 9 from Bell Labs](./interface-plan9.md)
	- [Interface para Terminal](./interface-terminal.md)
  - [Back-End](./backend.md)
- [Documentação do Software](./documentacao.md)
- [Especificação do Projeto](./especificacao-intro.md)
  - [Multi-Tenancy](./multi-tenancy.md)
  - [Banco de Dados Relacional](./banco-relacional.md)
    - [Executando migrations](./migrations.md)
  - [Banco de Dados Não-Relacional](./banco-nao-relacional.md)
    - [Coleções](./colecoes.md)
  - [Cache](./cache-redis.md)
  - [Mensageria](./mensageria-rabbitmq.md)
  - [Coleta de Logs](./logs-fluentd.md)
  - [Diagramas de Arquitetura](./diagramas-intro.md)
    - [Diagramas de Caso de Uso](./diagramas/casos-de-uso.md)
    - [Diagramas de Sequência](./diagramas-sequencia.md)
	  <!-- - [Inquilinos (`TENANCY`)]() -->
	  <!--   - [Listagem de nomes de inquilinos]() -->
	  <!-- 	- [Listagem de inquilinos]() -->
	  <!-- 	- [Criar inquilino]() -->
	  <!-- 	- [Desativar inquilino]() -->
	  - [Sessão (`SESSION`)](./diagramas-sequencia-sessao.md)
	    - [Login do usuário](./diagramas/login.md)
		- [Logoff do usuário](./diagramas/logoff.md)
	  - [Usuários (`USER`)](./diagramas-sequencia-usuarios.md)
		- [Cadastro de usuários](./diagramas/cadastro-usuarios.md)
	    - [Listagem de usuários](./diagramas/lista-usuarios.md)
		- [Consultar usuário](./diagramas/consultar-usuarios.md)
		- [Alteração de cadastro de usuários](./diagramas/alteracao-usuarios.md)
		- [Remoção de usuários](./diagramas/remocao-usuarios.md)
	  <!-- - [Produtos (`PRODUCT`)]() -->
	  <!-- 	- [Cadastro de produtos]() -->
	  <!--   - [Listagem de produtos]() -->
	  <!-- 	- [Consultar produto]() -->
	  <!-- 	- [Alteração de cadastro de produtos]() -->
	  <!-- 	- [Remoção de produtos]() -->
	  <!-- - [Estoque (`STOCK`)]() -->
	  <!--   - [Início de estoque]() -->
	  <!-- 	- [Consulta de estoque]() -->
	  <!-- 	- [Listagem de estoques]() -->
	  <!-- 	- [Entrada de estoque]() -->
	  <!-- 	- [Saída de estoque]() -->
	  <!-- 	- [Listagem de movimentações de um estoque]() -->
	  <!-- - [Relatórios (`REPORT`)]() -->
	  <!--   - [Listagem de Entidades na tela]() -->
	  <!-- 	- [Listagem de Entidades em PDF]() -->
	  <!-- - [Clientes (`CLIENT`)]() -->
	  <!--   - [Cadastro de clientes]() -->
	  <!-- 	- [Listagem de clientes]() -->
	  <!-- 	- [Consultar cliente]() -->
	  <!-- 	- [Alteração de cadastro de cliente]() -->
	  <!-- 	- [Remoção de clientes]() -->
	  <!-- - [Auditoria (`AUDIT`)]() -->
	  <!--   - [Consultar logs de auditoria]() -->
	  <!-- - [Comunicação Instantânea (`COMM`)]() -->
	  <!--   - [Enviar mensagem via WhatsApp]() -->
	  <!-- 	- [Enviar mensagem via Facebook Messenger]() -->
	  <!-- 	- [Enviar mensagem via Instagram]() -->
	  <!-- 	- [Enviar mensagem via Telegram]() -->
- [Compilação](./compilacao.md)
  - [Executar com recursos da máquina](./executar-maquina.md)
  - [Gerando imagens via Docker](./gerando-imagens.md)
  - [SonarQube e Quality Gates](./sonarqube.md)
- [Deploy](./deploy.md)
  - [Deploy via Docker Compose](./deploy-compose.md)
  - [Deploy via Docker Swarm + Vagrant](./deploy-swarm.md)
	- [Deploy via Docker Swarm + Docker Machine](./deploy-swarm-machine.md)
  - [Deploy via Kubernetes](./deploy-kubernetes.md)
    - [Monitoramento externo](./ferramentas-monitoramento-externas.md)
	  - [Swagger e RapiDoc](./monitoramento-swagger.md)
	  - [Grafana](./monitoramento-grafana.md)
	  - [MongoDB Compass](./monitoramento-mongodb.md)
	  - [RESP.app (para Redis)](./monitoramento-redis.md)
	  - [DBeaver CE (para PostgreSQL)](./monitoramento-postgres.md)
	  - [RabbitMQ](./monitoramento-rabbitmq.md)
	  - [ElasticSearch](./monitoramento-elasticsearch.md)
    - [Diagramas de arquitetura do cluster](./diagramas/diagrama-arquitetura.md)
	  - [REST](./diagramas/rest.md)
	  - [SESSION](./diagramas/session.md)
	  - [USER](./diagramas/user.md)
	  - [DISPATCH](./diagramas/dispatch.md)
	  - [PostgreSQL](./diagramas/postgresql.md)
	  - [MongoDB](./diagramas/mongodb.md)
	  - [Redis](./diagramas/redis.md)
	  - [RabbitMQ](./diagramas/rabbitmq.md)
	  - [Prometheus](./diagramas/prometheus.md)
	  - [Fluentd](./diagramas/fluentd.md)
	  - [Elasticsearch](./diagramas/elasticsearch.md)
	  - [Grafana](./diagramas/grafana.md)
	  - [Kibana](./diagramas/kibana.md)
	  <!-- - [PgAdmin4]() -->
	  <!-- - [Mongo Express]() -->
	  <!-- - [Redis Commander]() -->
- [Testes](./testes.md)
  - [Testes unitários](./testes-unitarios.md)
  - [Testes de integração](./testes-integracao.md)
  - [Testes de Carga](./testes-carga.md)
  
