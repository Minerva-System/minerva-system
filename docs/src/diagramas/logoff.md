# Logoff do usuário

<center>

```plantuml
@startuml
!theme amiga

actor       Usuário    as ator
boundary    FrontEnd   as frontend
boundary    API        as api
control     SESSION    as session
collections MongoDB    as mongo

ator     ->   frontend: Realização de logoff
frontend ->   api:      Requisição de encerramento de sessão
activate api
api      ->   session:  Remoção da sessão através do token
activate session
session  ->   mongo:    Verificação da existência da sessão
activate mongo
session  <--  mongo:    Dados da sessão
session  ->   mongo:    Remoção da sessão
session  <--  mongo:    Retorno da remoção de sessão
deactivate mongo
api      <--  session:  Sucesso na remoção da sessão
deactivate session
frontend <--  api:      Resposta de sucesso + remoção de cookies
deactivate api
ator     <--  frontend: Redirecionamento

@enduml
```

</center>
