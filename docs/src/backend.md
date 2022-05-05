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
- [ ] `minerva-store`: Implementação de utilitários de comunicação com
  datastore (Redis). Deve ser importado principalmente no módulo de login.

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
- [ ] `minerva-rest`: Servidor REST para comunicação com os demais módulos
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

| Serviço | Variável               | Porta |
|---------|------------------------|-------|
| USER    | `USER_SERVICE_PORT`    | 9010  |
| SESSION | `SESSION_SERVICE_PORT` | 9011  |
| PRODUCT | `PRODUCT_SERVICE_PORT` | 9012  |
| STOCK   | `STOCK_SERVICE_PORT`   | 9013  |
| REPORT  | `REPORT_SERVICE_PORT`  | 9014  |
| REST    | `REST_SERVICE_PORT`    | 9000  |


