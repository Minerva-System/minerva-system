# Listagem de usuários

<center>

```plantuml
@startuml
!theme amiga
actor       Usuário    as ator
boundary    FrontEnd   as frontend
boundary    API        as api
control     USERS      as users
control     SESSION    as session
collections Redis      as redis
collections MongoDB    as mongo
database    PostgreSQL as postgres

== Interação do usuário ==

ator     ->  frontend:     Acessa lista de usuários
frontend ->  api:          Requisita página de usuários
activate api

== Autenticação ==

api      ->  session:  Verifica validade da sessão
activate session
session  ->  redis:    Requisição dos dados da sessão em cache
activate redis
session  <-- redis:    Resultado da requisição do cache
deactivate redis

alt Se a sessão não estiver em cache
	session ->  mongo: Requisição dos dados de sessão
	activate mongo
	session <-- mongo: Dados da sessão
	deactivate mongo
	session ->  redis: Salva a sessão em cache
	activate redis
	session <-- redis: Sucesso
	deactivate redis
end

api   <-- session:  Aprovação da sessão
deactivate session

== Recuperação de dados de usuários ==

api -> users:              Requisita lista de usuários
activate users
users -> postgres:         Requisita dados de usuários
activate postgres
users <-- postgres:        Retorna dados de usuários
deactivate postgres
api <-- users:             Retorna lista de usuários
deactivate users

== Retorno da API ==

frontend <-- api:          Retorna página de usuários
deactivate api
ator <-- frontend:         Mostra usuários na interface

@enduml
```

</center>
