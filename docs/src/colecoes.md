# Coleções

Assim como no caso do banco de dados relacional, o banco de dados não-relacional
(criado através do MongoDB) também trabalha com um sistema _multi-tenant_, sendo
portanto representável como um banco de dados para cada cliente.

Ainda assim, para cada cliente, algumas coleções são essenciais de serem criadas
e configuradas até mesmo antes do primeiro acesso.

## Coleção `session`

A coleção `session` armazena documentos contendo dados de sessão de um usuário.
A responsabilidade de armazenar dados dos usuários é do banco de dados relacional,
assim como a responsabilidade de autenticá-los é do serviço `SESSION`. Esta coleção,
todavia, armazena os dados de autenticação após a realização de um login válido.

Cada documento nesta coleção possui um tempo de expiração de uma semana, o que
alinha-se com o tempo máximo de uma sessão do usuário ser, igualmente, uma semana.
A gerência desse tempo de expiração se dá através de um campo `creationDate` no
documento, que armazena um _timestamp_ indicando a data de início daquela sessão.
Caso o documento não possua esse campo, o MongoDB, por padrão, acaba não expirando-o.

A responsabilidade da definição e criação adequada do `creationDate` é do serviço
`SESSION`.
