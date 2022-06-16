# Cadastro de usuários

<center>

```plantuml
@startuml
!theme amiga
actor       Usuário    as ator
boundary    FrontEnd   as frontend
boundary    API        as api
control     USERS      as users
control     SESSION    as session
database    PostgreSQL as postgres
collections MongoDB    as mongo

ator     ->  frontend:     Cadastra novo usuário
frontend ->  api:          Requisita criação do novo cadastro
activate api
api      ->  session:      Verifica validade da sessão
activate session
session  ->  mongo:        Requisição dos dados de sessão
activate mongo
session  <-- mongo:        Dados da sessão
deactivate mongo
api      <-- session:      Aprovação da sessão
deactivate session

api      ->  users:        Requisição de criação de usuário
activate users
users    ->  postgres:     Verifica se usuário já existe
activate postgres
users    <-- postgres:     Retorno vazio
users    ->  postgres:     Insere dados de novo usuário
users    <-- postgres:     Dados do usuário recém-inserido
deactivate postgres
api      <-- users:        Dados do usuário criado
deactivate users
frontend <-- api:          Dados do usuário cadastrado
deactivate api
ator     <-- frontend:     Mensagem de sucesso

@enduml
```

</center>
