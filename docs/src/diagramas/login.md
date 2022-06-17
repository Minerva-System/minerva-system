# Login do usuário

Após o _login_ ser realizado, as sessões no Redis e no MongoDB obedecem um tempo
de vida limitado.

- **Redis**: Algumas horas.
- **MongoDB**: Uma semana.

Isso é gerenciado pelos próprios serviços. O MongoDB fará isso através do tempo
de vida da coleção para aquele banco, enquanto o Redis fará isso através do
tempo de vida da informação quando a mesma foi colocada em cache.

**TODO:** Seria mais adequado fazer com que `SESSION` requeira a `USERS` os
dados dos usuários, ao invés de recorrer ao banco?

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
session  <--  mongo:    ID da sessão
deactivate mongo
session  ->   redis:    Salva a sessão em cache
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

