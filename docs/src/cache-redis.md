# Cache via Redis

<center>
<img src="./redis-logo.png" alt="Redis" height="150"/>
</center>

A ferramenta Redis é extensamente usada no Minerva System para armazenamento de
informações em _cache_. Este capítulo descreve algumas das situações e entidades
para as quais o _cache_ é feito, e as regras de negócio envolvidas.

Lembre-se de que **o Redis é utilizado em ambientes onde o acesso é feito por
aplicações seguras**, o que significa que, por design, o mesmo deverá ser
capaz de armazenar informações sensíveis sem maiores problemas. Sendo um banco
de dados _in-memory_, deve-se esperar que sua reinicialização apague seus dados.


## Cache de sessão

Uma _sessão do usuário_ é armazenada no banco de dados não-relacional (MongoDB)
na forma de um documento, na coleção `session`, com um prazo de expiração de
uma semana.

Esse processo é feito durante o processo de login. Todavia, para evitar maior
estresse no serviço `SESSION` e no banco de dados não-relacional, o documento
do MongoDB é serializado para JSON, e armazenado como valor no Redis, com um
tempo de vida de 24 horas. A chave dessa informação é gerada a partir das
informações do _tenant_ e do ID do objeto no MongoDB.

Quando uma sessão for removida, além de realizar a remoção no MongoDB, o sistema
também fica a cargo de remover a sessão no banco de dados também.
