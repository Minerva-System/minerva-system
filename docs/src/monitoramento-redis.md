# RESP.app (para Redis)

<center>
<img src="./redis-logo.png" alt="Redis" height="150"/>
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

<center>
<img src="./resp-app.png" alt="RESP.app" width="600"/>
</center>

