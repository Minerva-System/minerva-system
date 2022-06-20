# Alteração do cadastro de usuários

<center>

```plantuml
@startuml
!theme amiga
actor       Usuário    as ator
boundary    FrontEnd   as frontend
boundary    API        as api
control     USER      as user
control     SESSION    as session
collections Redis      as redis
collections MongoDB    as mongo
database    PostgreSQL as postgres

== Interação do usuário ==

ator     ->  frontend:     Modifica dados do usuário
frontend ->  api:          Requisita alteração do usuário
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

== Verificação da existência do usuário ==

api      ->  user:        Requisição de alteração de usuário
activate user
user    ->  postgres:     Verifica se usuário já existe
activate postgres
user    <-- postgres:     Retorno com dados do usuário

== Alteração do usuário no banco de dados ==

user    ->  postgres:     Insere dados alterados do usuário
user    <-- postgres:     Dados do usuário recém-alterado
deactivate postgres
api      <-- user:        Dados do usuário alterado
deactivate user

== Retorno da API ==

frontend <-- api:          Dados do usuário alterado
deactivate api
ator     <-- frontend:     Mensagem de sucesso

@enduml
```

</center>
