# Login do usuário

<center>

```plantuml
@startuml
!theme amiga
actor       Usuário    as ator
boundary    FrontEnd   as frontend
boundary    API        as api
control     SESSION    as session
collections Redis      as redis
collections MongoDB    as mongo
database    PostgreSQL as postgres

== Interação do usuário ==

ator     ->   frontend: Realização de login
frontend ->   api:      Requisição de autenticação
activate api

== Criação da sessão ==

api      ->   session:  Requisição de token de sessão
activate session
session  ->   postgres: Requisição de dados de usuário
activate postgres
session  <--  postgres: Dados do usuário
deactivate postgres
session  ->   session:  Validação dos dados de login
session  ->   mongo:    Requisição de criação da sessão
activate mongo
session  <--  mongo:    ID da sessão\n(validade: uma semana)
deactivate mongo
session  ->   redis:    Salva a sessão em cache\n(validade: um dia)
activate redis
session  <--  redis:    Sucesso
deactivate redis
api      <--  session:  Token da sessão
deactivate session

== Retorno da API ==

frontend <--  api:      Resposta com token + cookies
deactivate api
ator     <--  frontend: Redirecionamento

@enduml
```

</center>

