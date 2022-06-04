# Deploy usando Docker Compose

![Docker Compose](./docker-compose.png)

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
diretório raiz deste projeto**.



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
seguintes portas para acesso:

| Porta  | Serviço   |
|--------|-----------|
| `80`   | Front-End |
| `9000` | API REST  |
| `8484` | pgAdmin4  |



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
