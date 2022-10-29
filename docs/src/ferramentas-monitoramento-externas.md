# Ferramentas para monitoramento externo

Esta seção demonstra algumas ferramentas que podem ser utilizadas para
monitoramento dos serviços cujo _deploy_ tenha sido realizado por meio de
Kubernetes.

A maioria dos serviços a seguir pode ser acessado através de _port-forward_.

## MongoDB Compass

<center>
<img src="./mongodb-compass.png" alt="MongoDB Compass" width="600"/>
</center>

O MongoDB Compass é uma ferramenta para desktop que permite inspecionar
documentos no MongoDB.

Para utilizá-lo, primeiramente realize _port-forward_ do MongoDB para
sua porta padrão na máquina atual:

```bash
kubectl port-forward -n minerva deployments/mongodb-deployment 27017:27017
```

Em seguida, conecte-se à instância do MongoDB através do host *`localhost:27017`*,
com usuários e senha padrão `root` e `mongo`.

## RESP.app

<center>
<img src="./resp-app.png" alt="RESP.app" width="600"/>
</center>

O RESP.app é uma ferramenta para desktop que permite inspecionar dados em cache
em um serviço Redis ou Redis Cluster.

Para acessar o Redis através dele, realize primeiro o _port-forward_ para
a porta padrão do Redis na sua máquina:

```bash
kubectl port-forward -n minerva statefulset/redis 6379:6379
```

Em seguida, clique em _Connect To Redis Server_. No campo de URL, digite o mesmo
texto do _hint_ apresentado (`redis://localhost:6379`), e então clique em _Import_.

Na próxima janela, dê um nome para a conexão e clique em OK. Não é necessário
segurança ou autenticação.

## DBeaver Community Edition

<!-- Mencionar port-forward -->

## RabbitMQ

<!-- Mencionar port-forward -->
<!-- Mencionar acesso web -->

## Grafana

<!-- Mencionar ingress -->
<!-- Migrar dashboards úteis para cá -->
