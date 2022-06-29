# Deploy via Docker Swarm + HashiCorp Vagrant

<center>
<img src="./swarm-vagrant.png" alt="Docker Swarm + HashiCorp Vagrant" width="400"/>
</center>

Além do deploy via Docker Compose, também é possível disponibilizar a stack do
Sistema Minerva em um _cluster_ do Docker Swarm.

Para tanto, é necessário inicializar o cluster. Isso pode ser feito, por
exemplo, com máquinas virtuais para finalidade de teste (neste caso, pode ser
utilizado o VirtualBox para prover essa facilidade).

Nesse capítulo, veremos como fazer isso de forma automatizada através de uma
configuração do Vagrant. Essa configuração criará máquinas virtuais e também
inicializará o sistema no _cluster_.

## Pré-requisitos

- Vagrant versão 2.2.19 ou superior;
- VirtualBox versão 6.1 ou superior.

Você pode utilizar outro _provider_ além de VirtualBox (como `libvirt`), mas
precisará alterar o arquivo `Vagrantfile`.

**NOTA:** Qualquer comando do Vagrant deve ser executado no diretório
`deploy/swarm`, para que o Vagrant tenha acesso ao `Vagrantfile`.

## Reinicializando o cluster

Caso você já tenha iniciado o _cluster_ com Vagrant, basta ir até o diretório
`deploy/swarm` e executar `vagrant up`. Isso reiniciará as máquinas virtuais,
mas também executará o `docker stack deploy` novamente para o arquivo de
configuração, o que forçará uma atualização em todos os serviços.

## Criando o cluster

Para criar o cluster, vá até o diretório `deploy/swarm` e execute o Vagrant.

```bash
cd deploy/swarm
vagrant up
```

Isso utilizará o arquivo `Vagrantfile` para criar sete máquinas virtuais
(dois _managers_ e dois _workers_), e também realizará automaticamente o
_deploy_ do sistema Minerva usando o arquivo `docker-stack.yml`.

Alguns arquivos extras serão criados na pasta. Eles dizem respeito respectivamente
ao IP do primeiro gerente e aos tokens de ingresso no _cluster_ para gerentes
e trabalhadores.

Em geral, a relação das máquinas virtuais do _cluster_ será:

- `manager01`: _Manager_, líder, inicializador original dos serviços;
- `manager02` e `manager03`: _Managers_ adicionais;
- `worker01` a `worker04`: _Workers_.

Para verificar o formato do _cluster_ e as informações acima, use o comando:

```bash
vagrant status
```

## Fazendo deploy do Sistema Minerva

Caso você realize modificações no arquivo `docker-stack.yml`, poderá
querer fazer _deploy_ novamente dos serviços de forma manual.

Para tanto, entre em qualquer um dos _managers_ via SSH. Por exemplo, para
o primeiro _manager_:

```bash
vagrant ssh manager01
```

O diretório `deploy/swarm` fica montado dentro de todas as máquinas virtuais
em `/vagrant` (que é mutável apenas durante a criação do _cluster_). Todavia,
você ainda poderá modificar os arquivos no _host_ e terá acesso a eles.

Para aplicar manualmente o arquivo `docker-stack.yml`:

```bash
# Em manager01
docker stack deploy --compose-file /vagrant/docker-stack.yml minerva
```

### Gerenciando a stack

Podemos gerenciar a stack facilmente dentro de uma VM manager.

Para listar as stacks ativas:

```bash
# Em manager01
docker stack ls
```

Se quisermos observar os serviços de uma stack em específico:

```bash
# Em manager01
docker stack services minerva
```

Ou, em último caso, se quisermos remover uma stack:

```bash
# Em manager01
docker stack rm minerva
```

### Acessando os serviços

Os serviços que podem ser acessados de forma externa estarão disponíveis
normalmente assim como no Docker Compose, porém sob um IP diferente.

Por padrão, todos os _managers_ possuem um IP começado com `172.20.20.1X`,
algo definido através do `Vagrantfile`. Os _workers_ terão um IP
iniciado com `172.20.20.10X`. A variável `X` será sempre um número
contado a partir de `1`.

Seguindo essas regras, as VMs possuirão os seguintes IPs:

| IP            | Hostname  |
|---------------|-----------|
| 172.20.20.11  | manager01 |
| 172.20.20.12  | manager02 |
| 172.20.20.13  | manager03 |
| 172.20.20.101 | worker01  |
| 172.20.20.102 | worker02  |
| 172.20.20.103 | worker03  |

Para acessar os serviços, use qualquer IP do _cluster_. A descoberta
do serviço será realizada através do _routing mesh_ do Docker Swarm.

Abaixo, temos uma relação das portas utilizadas para cada um dos serviços
disponíveis no _cluster_.

| Porta | Serviço                 |
|-------|-------------------------|
| 80    | Front-End               |
| 9000  | API REST                |
| 8484  | PgAdmin 4               |
| 8585  | Visualizador do cluster |
| 8686  | Mongo Express           |
| 8787  | Redis Commander         |


## Encerrando o serviço

Para encerrar todas as máquinas virtuais sem perder o estado das
mesmas, use:

```bash
vagrant suspend
```

Ou, se você desejar destruir o _cluster_ completamente:

```bash
vagrant destroy -f
```


