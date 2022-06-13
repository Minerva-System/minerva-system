# Gerando imagens via Docker


## Script para geração de imagens

Já existe um script separado para a geração das imagens. Para gerá-las, vá até
a raiz do repositório e execute o comando:

```bash
./generate_images.sh
```

Esse script foi especialmente feito para um console Bash, e pensado para
execução no Linux. No entanto, caso você esteja no Windows, poderá executá-lo
via Git Bash, MSYS2 ou similar, desde que seja possível utilizar o Docker
através da linha de comando.


## Gerando uma imagem em específico

Caso seja necessário, você poderá gerar uma imagem em específico de um projeto.

### Projetos Rust

Para qualquer projeto feito em Rust, poderá executar o seguinte comando a partir
da raiz do repositório:

```bash
docker image build -f build/Dockerfile \
	--target minerva_<projeto> \
	-t seu_username/minerva_<projeto>:latest \
	.
```

Lembre-se de substituir `<projeto>` pelo projeto em questão.

### Front-End

Para gerar o front-end da aplicação, feito com Flutter, teremos que usar um
Dockerfile diferente:

```bash
docker image build -f build/Dockerfile.frontend \
	-t seu_username/minerva_frontend:latest \
	.
```

### PgAdmin 4

A imagem com PgAdmin 4 é customizada com meros arquivos de configuração para
monitoramento do PostgreSQL. Por isso, também usa um Dockerfile diferente.

Veja também que ela é construída no diretório `build`.

```bash
docker image build -f build/Dockerfile.pgadmin \
	-t seu_username/minerva_pgadmin:latest \
	build
```


## Nomes e tags das imagens geradas

As imagens geradas pelos passos anteriores são geradas com nomes
específicos. Esses nomes serão muito úteis do ponto de vista do
envio dessas imagens para o DockerHub e do deploy via Docker
Compose, Docker Swarm e Kubernetes.

As imagens são sempre geradas com a tag `latest`.

A seguir, temos uma tabela relacionando os serviços com os nomes e tags
das imagens geradas. Veja que elas se relacionam, inclusive, com a forma
como essas imagens encontram-se no DockerHub (sob o /username/ `luksamuk`):

| Serviço      | Nome e tag da imagem               |
|--------------|------------------------------------|
| `frontend`   | `luksamuk/minerva_frontend:latest` |
| `rest`       | `luksamuk/minerva_rest:latest`     |
| `runonce`    | `luksamuk/minerva_runonce:latest`  |
| `users`      | `luksamuk/minerva_users:latest`    |
| `session`    | `luksamuk/minerva_session:latest`  |
| `pgadmin`    | `luksamuk/minerva_pgadmin:latest`  |
| `postgresql` | `postgres:14` (Não gerado)         |
| `mongodb`    | `mongo:5` (Não gerado)             |



## Subindo imagens para o DockerHub

Para enviar uma imagem para o DockerHub, primeiro é necessário se certificar de
que essa imagem possua uma _tag_ adequada. Por exemplo, supondo que acabamos de
gerar a imagem com a _tag_ 0.2.0 para o módulo `users`:

```bash
# Faça algo similar para cada uma das imagens
docker image tag luksamuk/minerva_users luksamuk/minerva_users:0.2.0
```

Em seguida, poderemos enviar todas as tags das imagens para o DockerHub.

```bash
docker image push -a luksamuk/minerva_frontend
docker image push -a luksamuk/minerva_rest
docker image push -a luksamuk/minerva_runonce
docker image push -a luksamuk/minerva_users
docker image push -a luksamuk/minerva_session
docker image push -a luksamuk/minerva_pgadmin
```
