# Deploy

As sessões a seguir tratarão do deploy da aplicação, ou mais especificamente, de
formas escaláveis de subir a aplicação num ambiente que simule produção.

Este capítulo trata de três tipos específicos de deploy:

- Usando Docker Compose;
- Usando Docker Swarm;
- Usando Kubernetes, através do Minikube.

O Docker Compose poderá ser utilizado em situações de teste e desenvolvimento,
especialmente porque seu arquivo de configuração sempre aponta para a imagem
com tag `latest` de todos os contêineres (o que significa que utilizará
as imagens que tiverem sido recém-geradas na máquina).

O Docker Swarm trabalha de forma similar ao Compose em sua configuração, porém
utilizaremos a ferramenta `docker stack`, além de certa configuração manual,
para subir um cluster com `docker-machine`, que pode também ser configurado de
forma doméstica. A configuração poderá então ser usada para orquestração de
contêineres.

Já o Kubernetes, utilizando uma máquina virtual KVM2 através do Minikube,
possibilita uma orquestração de contêineres ainda mais flexível. Esta configuração
é utilizada também para um ambiente local, mas será a forma mais próxima de
colocar o sistema em produção.
