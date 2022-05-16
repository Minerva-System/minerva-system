# Deploy usando Kubernetes (e Minikube)

![Kubernetes](./kubernetes-logo.webp)

Você pode realizar deploy do projeto usando Kubernetes. Nos passos a
seguir, será mostrado como realizar um deploy usando a ferramenta
Minikube, para instalação do Kubernetes localmente.





## Introdução

Kubernetes é uma ferramenta sofisticada de orquestração de contêineres.
O Minerva System é planejado para que seu deploy seja feito utilizando
o Kubernetes.

Para realizar a configuração localmente, fui utilizada a ferramenta
Minikube. Portanto, os comandos aqui abordados partem do pressuposto
de uma instalação local do Kubernetes via Minikube, e podem ser 





## Objetivo

O deploy usando Kubernetes é planejado desde o início do projeto, sendo
uma das formas de estado da arte de deploy de aplicações web. Para
simular este cenário, utilizamos uma instalação local do Kubernetes
via Minikube.

Ainda que Minikube não seja exatamente um servidor do Kubernetes em
produção, boa parte do que será aqui discutido poderá ser utilizado
no momento do deploy para produção.




## Dependências

Você precisará ter instalado:
- Docker versão 20.10 ou superior;
- Docker Compose versão 2.2.3 ou superior;
- Kubectl versão 1.23.3 ou superior;
- [Minikube](https://minikube.sigs.k8s.io/docs/) versão 1.24.0 ou
  superior;
- [k9s](k9scli.io) versão 0.25.18 ou superior (opcional).

A instalação do k9s é opcional, sendo uma ferramenta de monitoramento
e gerencialmento do Kubernetes via linha de comando.



### Iniciando o Minikube

Caso você esteja testando localmente, comece executando o Minikube.

```bash
minikube start
```

Se você quiser parar o Minikube:

```bash
minikube stop
```




## Geração das imagens

Antes de realizar deploy, é preciso gerar as imagens de cada um dos
serviços. No caso específico do Minikube, é preciso também que essas
imagens localmente construídas sejam carregadas para dentro deste
serviço.



### Gerando imagens com Docker Compose

Você poderá gerar as imagens dos serviços usando a própria
infraestrutura de geração de imagens do Docker Compose, o que é
recomendado. Para tanto, veja a seção
[geração das imagens](./deploy-compose.md#gerando-imagens)
na página de configuração do Docker Compose.

Esse passo é essencial para serviços e jobs marcados com
`imagePullPolicy` igual a `Never`.



### Compartilhando as imagens com o Minikube

Caso você esteja utilizando o Minikube, precisará carregar as
imagens no ambiente do mesmo, do contrário ainda terá erros
relacionados ao `imagePullPolicy`. Isso pode ser feito com um
comando como mostrado a seguir:

```bash
minikube image load <nome-imagem>:<tag>
```

Por exemplo, para carregar a imagem do Front-End, use o comando
`minikube image load minerva_frontend:latest`.

Você poderá também informar mais de uma imagem de uma só vez,
por exemplo:

```bash
minikube image load \
	 minerva_rest:latest \
	 minerva_frontend:latest \
	 minerva_pgadmin:latest \
	 minerva_users:latest \
	 minerva_runonce:latest
```




## Disposição da configuração

A configuração do Sistema Minerva para Kubernetes foi originalmente
produzida através de conversão direta da configuração do Docker
Compose, mais especificamente através da ferramenta
[Kompose](https://kompose.io), e então modificada para suprir algumas
necessidades do sistema.

De forma geral, no Kubernetes, temos que:

- Um **node** refere-se a uma instalação do Kubernetes em uma máquina.
- Um **pod** é um grupo de um ou mais contêineres, que compartilha
  armazenamento e recursos de rede.
- Um **deployment** é a descrição de um *pod*, mais especificamente
  o seu comportamento e suas características. Também pode ser usado
  para gerenciar réplicas de *pods* (ou *ReplicaSets*).
- Um **service** é uma configuração de acessibilidade para os *pods*,
  mais especificamente agindo como uma forma de localizá-los na rede.
  Em outras palavras, age como um gerenciador de DNS.

Assim, temos a seguinte configuração de serviços:

| Serviço      | Tipo           | Réplicas padrão |
|--------------|----------------|-----------------|
| `frontend`   | *LoadBalancer* | 2               |
| `pgadmin`    | *LoadBalancer* | 1               |
| `rest`       | *LoadBalancer* | 3               |
| `runonce`    | *Job*          | **Sempre 1**    |
| `users`      | *ClusterIP*    | 3               |
| `postgresql` | *ClusterIP*    | **Sempre 1**    |

- Serviços *LoadBalancer* são acessíveis via IP externo.
- *Jobs* são executados no início da configuração do *node*.
- Serviços *ClusterIP* são acessíveis apenas na rede interna do *node*.




## Executando e gerenciando o cluster

Para iniciar a configuração, posto que as imagens dos serviços
estejam à disposição do Kubernetes (ou do Minikube), vá até o diretório
raiz do projeto e execute:

```bash
kubectl create -f kubernetes.yaml
```

Caso você realize alguma alteração no arquivo `kubernetes.yaml` e
queira aplicá-la, utilize:

```bash
kubectl apply -f kubernetes.yaml
```

Caso você queira deletar o cluster posteriormente, utilize o comando
similar no mesmo diretório:

```bash
kubectl delete -f kubernetes.yaml
```



### Monitorando via k9s

![k9s](./k9s.png)

Uma das ferramentas possíveis de se utilizar para monitorar o cluster
é o `k9s`.

A ferramenta utiliza uma edição modal, muito parecida com o editor
Vim. Os comandos possuem um sistema de autocompletar e são também
mostrados na tela. Alguns comandos interessantes de serem utilizados
são:

- `:q`: Sair da aplicação.
- `:po`: Lista de *pods*.
- `:svc`: Lista de *services*.
- `:deployment`: Lista de *deployments*.

Você poderá usar o `k9s` para visualizar logs e também para modificar
algumas propriedades mais avançadas também.




### Logs

Para visualizar logs completos de cada *pod*, primeiramente descubra
o nome do *pod* que você deseja observar:

```bash
kubectl get pods
```

Um exemplo da saída deste comando poderia ser:

```text
NAME                          READY   STATUS      RESTARTS   AGE
frontend-86c7955bdc-7zv4g     1/1     Running     0          43m
pgadmin-76c5d7ccf7-4wk7g      1/1     Running     0          43m
postgresql-6f8b685668-94qbs   1/1     Running     0          43m
rest-57f984cb4c-25s9h         1/1     Running     0          42m
runonce--1-kl4rb              0/1     Completed   0          43m
users-549f5775c4-8fh9x        1/1     Running     0          42m
```

Para ver os logs completos da execução atual de um pod qualquer,
use o nome completo do *pod*, como mostrado acima:

```bash
kubectl logs runonce--1-kl4rb
```

Caso o *pod* não tenha sido encerrado corretamente da última vez,
você poderá ver os logs anteriores com a flag `--previous`:

```bash
kubectl logs --previous runonce--1-kl4rb
```





## Acessando serviços com Minikube

Caso você queira descobrir o endereço para um serviço de tipo
*LoadBalancer*, você poderá descobrir o IP usando o Minikube:

```bash
minikube service rest --url
```

Supondo que este comando retorne um valor para o serviço `rest` como
`http://192.168.49.2:30617`, agora será possível fazer uma requisição
REST comum diretamente neste endereço:

```bash
curl http://192.168.49.2:30617/minerva/users
```

```text
[{"id":1,"login":"admin","name":"Administrator","email":null}]
```

