# Gerando imagens via Docker


## Script para geração de imagens

Já existe um script separado para a geração das imagens (com _tags_
apropriadas). Para gerá-las, vá até a raiz do repositório e execute o comando:

```bash
./generate_images.sh
```

Esse script foi especialmente feito para um console Bash, e pensado para
execução no Linux. No entanto, caso você esteja no Windows, poderá executá-lo
via Git Bash, MSYS2 ou similar, desde que seja possível utilizar o Docker
através da linha de comando.


## Gerando uma imagem em específico

Caso seja necessário, você poderá gerar uma imagem em específico de um projeto.

Para qualquer projeto, poderá executar o seguinte comando a partir
da raiz do repositório:

```bash
docker buildx build \
	-f build/Dockerfile \
	--target minerva_<projeto> \
	-t seu_username/minerva_<projeto>:latest \
	.
```

Lembre-se de substituir `<projeto>` pelo projeto em questão.

### Criando uma tag para a imagem

Todas as imagens são geradas automaticamente com _tags_ de acordo com o projeto
do qual está sendo gerado (arquivos `Cargo.toml` e `pubspec.yaml`).

Se você estiver gerando as imagens manualmente, poderá definir uma _tag_ como
no exemplo a seguir:

```bash
# Faça algo similar para cada uma das imagens
docker image tag \
	seu_username/minerva_user \
	seu_username/minerva_user:0.2.0
```


## Nomes e tags das imagens geradas

As imagens geradas pelos passos anteriores são geradas com nomes
específicos. Esses nomes serão muito úteis do ponto de vista do
envio dessas imagens para o DockerHub e do deploy via Docker
Compose, Docker Swarm e Kubernetes.

As imagens são sempre geradas com a tag `latest`, mas também
receberão _tags_ de acordo com seus arquivos de projeto (`Cargo.toml`
e `pubspec.yaml`).

A seguir, temos uma tabela relacionando os serviços com os nomes e tags
das imagens geradas. Veja que elas se relacionam, inclusive, com a forma
como essas imagens encontram-se no DockerHub (sob o _username_ `luksamuk`):

| Serviço      | Nome e tag da imagem                 |
|--------------|--------------------------------------|
| `frontend`   | `luksamuk/minerva_frontend:latest`   |
| `rest`       | `luksamuk/minerva_rest:latest`       |
| `runonce`    | `luksamuk/minerva_runonce:latest`    |
| `user`       | `luksamuk/minerva_user:latest`       |
| `session`    | `luksamuk/minerva_session:latest`    |
|--------------|--------------------------------------|
| `pgadmin`    | `dpage/pgadmin4:latest` (Não gerado) |
| `postgresql` | `postgres:15` (Não gerado)           |
| `mongodb`    | `mongo:4` (Não gerado)               |



## Subindo imagens para o DockerHub

Para enviar uma imagem para o DockerHub, primeiro é necessário se certificar de
que essa imagem possua uma _tag_ adequada.

Em seguida, poderemos enviar todas as tags das imagens para o DockerHub.

```bash
docker image push -a luksamuk/minerva_frontend
docker image push -a luksamuk/minerva_rest
docker image push -a luksamuk/minerva_runonce
docker image push -a luksamuk/minerva_user
docker image push -a luksamuk/minerva_session
```

## Geração de imagens cross-platform

O script `generate_and_push.sh` gera imagens _cross-platform_ e de forma
otimizada, sendo o script preerido a ser executado para enviar imagens para
o DockerHub.

Use esse script se a intenção for compilar para mais arquiteturas ou gerar
versão oficial, de forma otimizada.

