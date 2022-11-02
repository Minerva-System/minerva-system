# Documentação do Software

Este capítulo lista links para a documentação das partes pertinentes ao código
do Minerva System.

Por padrão, a documentação é escrita em Inglês, e pode ser muito pertinente
durante a implementação de novas partes do sistema.

Não se esqueça de consultar estes documentos com frequência.

## API

- [Documentação da API](https://documenter.getpostman.com/view/17061755/Uyxoi4MU) \
  Documentação da API REST (Postman, em Inglês).

## Serviços externos

- [FRONTEND](./doc/minerva_frontend/index.html) \
  _Front-End_ do Minerva System.
- [REST](./doc/minerva_rest/index.html) \
  _Gateway_ REST para acesso aos demais serviços.

## Microsserviços

- [RUNONCE](./doc/minerva_runonce/index.html) \
  Utilitário de configuração inicial do sistema durante um deploy.
- [SESSION](./doc/minerva_session/index.html) \
  Serviço de gerenciamento de sessão de usuário.
- [USER](./doc/minerva_user/index.html) \
  Serviço de gerenciamento de usuários.
- [DISPATCH](./doc/minerva_dispatch/index.html) \
  Serviço de consumo de filas do RabbitMQ e despacho de operações.
- PRODUCT _(não implementado)_ \
  Serviço de gerenciamento de produtos.
- REPORT _(não implementado)_ \
  Serviço de gerenciamento e emissão de relatórios.
- STOCK _(não implementado)_ \
  Serviço de gerenciamento de estoques de produtos.
- CLIENT _(não implementado)_ \
  Serviço de gerenciamento de clientes.
- AUDIT _(não implementado)_ \
  Serviço de gerenciamento de _logs_ de auditoria.
- TENANCY _(não implementado)_ \
  Serviço de gerenciamento de inquilinos.
- COMM _(não implementado)_ \
  Serviço de gerenciamento de comunicações via mensagem instantânea.

## Bibliotecas

- [DATA](./doc/minerva_data/index.html) \
  Biblioteca de manipulação de DTOs e conversões de dados.
- [RPC](./doc/minerva_rpc/index.html) \
  Biblioteca de implementação de Protocol Buffers, mensagens gRPC e afins.
- [CACHE](./doc/minerva_cache/index.html) \
  Biblioteca para uso e acesso ao cache via serviço Redis.
- [BROKER](./doc/minerva_broker/index.html) \
  Biblioteca para uso, acesso e configuração do serviço RabbitMQ e mensageria.


