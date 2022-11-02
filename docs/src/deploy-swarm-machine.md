# Deploy via Docker Swarm + Docker Machine

<center>
<img src="./docker-swarm.png" alt="Docker Swarm" width="200"/>
</center>

Outra forma de realizar _deploy_ usando o Docker Swarm é através do Docker
Machine, um utilitário capaz de criar máquinas virtuais com Docker já
instalado a partir de uma imagem Linux especial.

O Docker Machine é uma ferramenta defasada, mas pode ser uma alternativa quando
for do interesse do programador fazer um trabalho mais manual. Por ser o método
utilizado antes do uso do Vagrant, esta página existe como referência ao uso
da ferramenta.

## Pré-requisitos

- Docker Machine versão 0.16.2;
- VirtualBox versão 6.1 ou superior.

### Instalando o Docker Machine no Linux

Como o Docker Machine é uma ferramenta defasada, é necessário executar
passos como os a seguir para instalar:

```bash
base=https://github.com/docker/machine/releases/download/v0.16.2
curl -L $base/docker-machine-$(uname -s)-$(uname -m) >/tmp/docker-machine
sudo install /tmp/docker-machine /usr/local/bin/docker-machine
```

Para instalar os drivers de KVM2, caso você prefira utilizá-los:

```bash
base=https://github.com/praveenkumar/machine-kvm2-driver/releases/download/v0.11.0
curl -L $base/docker-machine-driver-kvm2 >/tmp/docker-machine-driver-kvm2
sudo install /tmp/docker-machine-driver-kvm2 /usr/local/bin/docker-machine-driver-kvm2
```


## Reinicializando o cluster

Caso você já tenha construído um cluster e feito _deploy_ via Docker Machine
anteriormente, é bem provável que você não precise fazer a maioria do trabalho.
Você poderá simplesmente reiniciar as máquinas virtuais:

```bash
docker-machine start `docker-machine ls "minerva-*" -q`
```

Existe a possibilidade de as máquinas virtuais não conseguirem se reconhecer
por uma mudança de IP. Se isso ocorrer, reconfigure o _cluster_ manualmente,
gerando os _tokens_ para cada máquina virtual e inserindo-as no _cluster_.


## Criando o cluster

Se você ainda não tiver um _cluster_ criado, poderá criar o _cluster_ através
da ferramenta Docker Machine. Comece criando uma máquina virtual chamada
`minerva-vm1`, que será nosso inicializador do _cluster_.

```bash
docker-machine create -d virtualbox --swarm-master minerva-vm1
```

### Iniciando o cluster

Vamos começar iniciando o Docker Swarm na primeira máquina virtual.

Para acessar o console de uma máquina virtual via SSH, use também o Docker
Machine para isso.

```bash
docker-machine ssh minerva-vm1
```

Para iniciar o cluster, precisamos descobrir também o IP dessa máquina virtual.
Você poderá ver o IP de uma máquina virtual em específico via Docker Machine
também, em outro console:

```bash
docker-machine ip minerva-vm1
```

Voltando ao console da VM, vamos iniciar o Docker Swarm.

```bash
# Em minerva-vm1
docker swarm init --advertise-addr <IP>
```

### Criando mais managers

Uma arquitetura básica de managers e workers do Swarm, para que o algoritmo de
consenso RAFT opere como esperado, poderia envolver três managers e dois workers
-- portanto, cinco máquinas virtuais.

Vamos criar mais duas máquinas virtuais que vão servir de managers (`minerva-vm2`
e `minerva-vm3`):

```bash
docker-machine create -d virtualbox --swarm-master minerva-vm2
docker-machine create -d virtualbox --swarm-master minerva-vm3
```

Para adicionar essas VMs no _cluster_, vamos obter o token de entrada no _cluster_
para managers, que será um mero comando do console. Copiamos esse comando e colamos
no console das duas máquinas virtuais recém-criadas.

```bash
# Em minerva-vm1
docker swarm join-token manager

# Em minerva-vm2 e minerva-vm3: Cole o comando
docker swarm join --token...
```

### Criando workers

Criaremos mais duas máquinas virtuais com o Swarm preparado, mas dessa vez, vamos
prepará-las para serem meros workers:

```bash
docker-machine create -d virtualbox --swarm minerva-vm4
docker-machine create -d virtualbox --swarm minerva-vm5
```

O princípio para adicionar workers no _cluster_ é o mesmo, porém usaremos um comando
diferente para gerar o token. Geramos esse comando, copiamos e colamos no console
das VMs `minerva-vm4` e `minerva-vm5`.

```bash
# Em minerva-vm1
docker swarm join-token worker

# Em minerva-vm4 e minerva-vm5: Cole o comando
docker swarm join --token...
```

### Verificando a topologia do cluster

Vamos verificar a topologia do _cluster_. Podemos observar a atividade das máquinas
virtuais diretamente através do Docker Machine:

```bash
docker-machine ls
```

```
NAME          ACTIVE   DRIVER       STATE     URL                         SWARM   DOCKER      ERRORS
minerva-vm1   -        virtualbox   Running   tcp://192.168.99.108:2376           v19.03.12
minerva-vm2   -        virtualbox   Running   tcp://192.168.99.109:2376           v19.03.12
minerva-vm3   -        virtualbox   Running   tcp://192.168.99.110:2376           v19.03.12
minerva-vm4   -        virtualbox   Running   tcp://192.168.99.111:2376           v19.03.12
minerva-vm5   -        virtualbox   Running   tcp://192.168.99.112:2376           v19.03.12
```

Para avaliarmos o _cluster_ em si e a forma como os nós se conectam, poderemos ver a
topologia dos nós diretamente dentro da primeira VM:

```bash
# Em minerva-vm1
docker node ls
```

```
ID                            HOSTNAME            STATUS              AVAILABILITY        MANAGER STATUS      ENGINE VERSION
exgmsiju6pnrl01tt33n5guui *   minerva-vm1         Ready               Active              Leader              19.03.12
cpxtnhalvu9tat9ek4n1n0117     minerva-vm2         Ready               Active              Reachable           19.03.12
p2v7v8ac93wuhwhcdsjl00p8y     minerva-vm3         Ready               Active              Reachable           19.03.12
jihrf6wgm145xzr0pdb6tnrck     minerva-vm4         Ready               Active                                  19.03.12
b1wfgme22m14pmjceo8ktn1hj     minerva-vm5         Ready               Active                                  19.03.12
```

Outra opção interessante é acompanhar também os serviços e os contêineres criados:

```bash
# Em minerva-vm1
docker service ls
docker container ls
```

### Fazendo backup do cluster

Caso você queira fazer backup da topologia do _cluster_, lembre-se de copiar o
diretório `/var/lib/docker/swarm` em `minerva-vm1`.

```bash
# Em minerva-vm1
sudo cp -r /var/lib/docker/swarm ./swarm
sudo chown -R $USER ./swarm

# No host
docker-machine scp -r minerva-vm1:/home/docker/swarm localhost:~/swarm-backup
```

## Fazendo deploy do Sistema Minerva

Para fazer _deploy_ do sistema, dado que o _cluster_ esteja configurado, basta
reutilizar o arquivo preparado para isso no repositório do Sistema Minerva.

Copiamos o arquivo para dentro da VM principal e então realizamos deploy:

```bash
docker-machine scp localhost:./deploy/swarm/docker-stack.yml minerva-vm1:/home/docker/docker-stack.yml

# Em minerva-vm1
docker stack deploy --compose-file docker-stack.yml minerva
```

### Gerenciando a stack

Podemos gerenciar a stack facilmente dentro de uma VM manager.

Para listar as stacks ativas:

```bash
# Em minerva-vm1
docker stack ls
```

Se quisermos observar os serviços de uma stack em específico:

```bash
# Em minerva-vm1
docker stack services minerva
```

Ou, em último caso, se quisermos remover uma stack:

```bash
# Em minerva-vm1
docker stack rm minerva
```

### Acessando os serviços

Para visualizar os serviços, primeiro visualize o IP das Docker Machines:

```bash
docker-machine ls
```

É possível usar o IP de qualquer Docker Machine, neste ponto. Basta utilizar
as portas certas:

| Porta | Serviço                 |
|-------|-------------------------|
| 80    | Front-end               |
| 9000  | API REST                |
| 8484  | PgAdmin 4               |
| 8585  | Visualizador do cluster |
| 8686  | Mongo Express           |
| 8787  | Redis Commander         |
| 5672  | RabbitMQ (Serviço)      |
| 15672 | RabbitMQ (Gerenciador)  |

## Encerrando o serviço

Caso você queira parar todas as máquinas virtuais, use o comando a seguir:

```bash
docker-machine stop `docker-machine ls "minerva-*" -q`
```

Ou, se você quiser remover realmente as máquinas virtuais:

```bash
docker-machine rm `docker-machine ls "minerva-*" -q`
```
