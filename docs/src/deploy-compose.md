# Deploy usando Docker Compose

<center>
<img src="./docker-compose.png" alt="Docker Compose" width="350"/>
</center>

Você pode realizar deploy do projeto usando Docker Compose. Todavia,
esta não é a forma mais recomendada de realização de deploy.



## Introdução

Docker Compose é uma ferramenta simples de orquestração de contêineres.
Para o Minerva System, é principalmente uma forma de **testar** a forma
como o serviço se comporta em rede.



## Objetivo

O deploy usando Docker Compose é útil principalmente do ponto de vista
da geração das imagens das aplicações dos microsserviços do Minerva
System, mas também não é a forma mais recomendada de colocar o sistema
em produção, porque não prevê fatores de escalabilidade como o deploy
usando Kubernetes.

Utilize esta forma principalmente quando quiser avaliar o comportamento
do sistema no que tange a interconexões entre os serviços numa rede
virtual.




## Dependências

Você precisará ter:

- Docker versão 20.10 ou superior;
- Docker Compose versão 2.2.3 ou superior;
- As imagens do projeto (se não estiverem localmente disponíveis,
  serão baixadas).

Além disso, **todos os comandos a seguir devem ser executados no
diretório `deploy/coompose` deste projeto**.





## Preparando-se para a execução dos serviços

Primeiramente, você deverá se preparar para a geração de arquivos de
log em cada serviço. Para tanto, no Linux, execute o script `make_log_dir.sh`.
Esse script criará uma pasta `log` que será montada para o usuário
com UID 1000, correspondente ao usuário `appuser` na maioria das imagens
dos serviços do Minerva System.
 


## Executando dependências

O Minerva System depende essencialmente de quatro serviços de terceiros:

- PostgreSQL;
- MongoDB;
- Redis;
- RabbitMQ.

É possível realizar a execução desses serviços através de algum provedor
que os facilite, mas, para eventuais testes locais, você poderá usar uma
configuração de Docker Compose específica ou utilizar diretamente esses
serviços caso estejam hospedados em um cluster Kubernetes, através de
port-forward.

### Executando via Docker Compose

Para executar os serviços na máquina atual, você deverá navegar até
a pasta `services` e iniciar o Docker Compose:

```bash
cd services
docker compose up
```

Os serviços estarão expostos nas seguintes portas:

| Porta | Serviço                |
|-------|------------------------|
| 8484  | PgAdmin 4              |
| 8686  | Mongo Express          |
| 8787  | Redis Commander        |
| 5672  | RabbitMQ (Serviço)     |
| 15672 | RabbitMQ (Gerenciador) |

O gerencialmento funcionará de forma similar aos serviços em si,
portanto, para maiores informações sobre o uso do Compose, veja
a seção _Executando os serviços_.




### Usando Port Forward do Kubernetes

Em seguida, caso você não queira executar os serviços essenciais
mais pesados em termos de recursos (PostgreSQL, MongoDB, Redis e
RabbitMQ), poderá reaproveitá-los caso tenha realizado deploy dos
mesmos em Kubernetes. Para tanto, você poderá usar um script
preparado que realiza esse processo. Veja que esse script assume
que você possua a ferramenta `kubectl` com acesso padrão configurado
para o cluster que seja seu _target_.

O script encontra-se excepcionalmente em `helpers/port-forwards.sh`,
na raiz do projeto.




## Executando os serviços

Para executar os serviços usando Docker Compose, use o seguinte
comando:

```bash
docker compose up
```

Caso você queira desligar o funcionamento dos serviços da sessão
atual do console, poderá executá-los em forma de *daemon*:

```bash
docker compose up -d
```

Neste caso em específico, para `localhost`, estarão abertas as
seguintes portas para acesso aos serviços:

| Porta | Serviço                    |
|-------|----------------------------|
| 9000  | API REST (endpoint `/api`) |
| 9010  | USER                       |
| 9011  | SESSION                    |


## Acompanhando logs

Para acompanhar os logs de um deploy via *daemon* ou de um outro
console, você poderá realizá-lo através do comando:

```bash
docker compose logs -f
```

Caso seja necessário acompanhar os logs de apenas um serviço:

```bash
docker compose logs -f <servico>
```

Lembre-se de que o nome do serviço em questão deverá ser informado
como listado em `docker-compose.yml`.


### Reiniciando um único serviço

Você poderá reiniciar um único serviço, caso tenha recompilado a imagem
do mesmo, por exemplo.

Nesse caso, basta usar o seguinte comando:

```bash
docker compose up -d --no-deps <servico>
```

Caso você queira incluir o passo de recompilação da imagem:

```bash
docker compose up -d --no-deps --build <servico>
```




## Encerrando os serviços

Para encerrar imediatamente o serviço, execute o seguinte comando:

```bash
docker compose down
```

Caso você queira também remover os volumes associados aos serviços
(por exemplo, nocaso do PostgreSQL e do pgAdmin), use este comando
em vez do anterior:

```bash
docker compose -v down
```
