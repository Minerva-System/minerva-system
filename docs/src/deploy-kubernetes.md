# Deploy usando Kubernetes (e Minikube)

![Kubernetes](./kubernetes-logo.webp)

Você pode realizar deploy do projeto usando Kubernetes. Nos passos a
seguir, será mostrado como realizar um deploy usando a ferramenta
Minikube, para instalação do Kubernetes localmente.

**ATENÇÃO:** Para detalhes não dispostos nesta página, veja a
[documentação oficial do Kubernetes](https://kubernetes.io/docs/home/).





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

Você também pode acessar facilmente um dashboard web do Kubernetes,
operando sob o Minikube, caso não queira usar o k9s posteriormente:

```bash
minikube dashbord
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

Os arquivos de configuração do Kubernetes estão desmembrados em suas
devidas partes, seguindo ligeiramente o âmbito dos *services* e dos
*deployments*, exceto quando não aplicável, e podem ser encontrados
no diretório `build/kubernetes`.

Assim, temos a seguinte configuração de serviços:

| Serviço      | Tipo           | Réplicas padrão |
|--------------|----------------|-----------------|
| `frontend`   | *LoadBalancer* | 2               |
| `pgadmin`    | *LoadBalancer* | 1               |
| `rest`       | *LoadBalancer* | 3               |
| `runonce`    | *Job*          | **Sempre 1**    |
| `users`      | *ClusterIP*    | 3               |
| `postgresql` | *ClusterIP*    | **Sempre 1**    |

Devemos considerar também, sobre os tipos dos serviços:

- Serviços *LoadBalancer* são acessíveis via IP externo.
- *Jobs* são executados no início da configuração do *node*.
- Serviços *ClusterIP* são acessíveis apenas na rede interna do *node*.




## Executando e gerenciando o cluster

Para iniciar a configuração, posto que as imagens dos serviços
estejam à disposição do Kubernetes (ou do Minikube), vá até o diretório
raiz do projeto e execute:

```bash
kubectl create -f build/kubernetes
```

Isso criará o *node* com base em todos os arquivos `.yaml` existentes
no diretório `build/kubernetes`.


Caso você realize alguma alteração nos arquivos e queira aplicá-las,
utilize:

```bash
kubectl apply -f build/kubernetes
```

Da mesma forma, é possível aplicar as alterações de um único arquivo.
Por exemplo:

```bash
kubectl apply -f build/kubernetes/rest-deployment.yaml
```

Caso você queira deletar o cluster posteriormente, utilize o comando
similar no mesmo diretório:

```bash
kubectl delete -f build/kubernetes
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

Há duas formas de acessar serviços pelo Minikube: através de
*NodePort* e de *LoadBalancer*. Este último envolve a ideia
de *tunelamento* via Minikube.



### Acessando via *NodePort*

Caso você queira descobrir o endereço para um serviço de tipo
*LoadBalancer*, você poderá descobrir o IP usando o Minikube.
Este método basicamente utiliza a ideia de *NodePort* para
redirecionar o tráfego para a aplicação:

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


### Acessando via *LoadBalancer*

Primeiramente, caso você esteja executando o Minikube, abra um terminal
avulso (que terá a execução bloqueada pelo próximo comando) e digite:

```bash
minikube tunnel
```

**Note que este comando pode exigir a senha do seu usuáio.**

Isso fará com que todos os nós do tipo *LoadBalancer* recebem IPs externos
que podem ser acessados imediatamente. Para verificar esses IPs externos,
verifique os serviços ativos (isso também pode ser observado pelo k9s):

```bash
kubectl get svc
```

Um exemplo de saída do comando:

```text
NAME         TYPE           CLUSTER-IP       EXTERNAL-IP      PORT(S)          AGE
frontend     LoadBalancer   10.108.114.123   10.108.114.123   80:32613/TCP     31m
kubernetes   ClusterIP      10.96.0.1        <none>           443/TCP          8h
pgadmin      LoadBalancer   10.99.60.133     10.99.60.133     8484:30065/TCP   31m
postgresql   ClusterIP      10.99.167.49     <none>           5432/TCP         31m
rest         LoadBalancer   10.111.150.132   10.111.150.132   9000:31522/TCP   31m
users        ClusterIP      10.97.99.55      <none>           9010/TCP         31m
```

No caso acima, podemos verificar, por exemplo, que requisições REST devem ser
direcionadas para o caminho `http://10.111.150.132:9000`, como observado pelas
colunas `EXTERNAL-IP` e `PORT(S)`.

Portanto:

```bash
curl http://10.111.150.132:9000/minerva/users
```

```text
[{"id":1,"login":"admin","name":"Administrator","email":null}]
```

Caso haja portas pendentes de clientes de tunelamento do Minikube que não tenham
sido liberadas, você poderá liberar as portas manualmente com:

```bash
minikube tunnel --cleanup
```



## Escalando *deployments*

Quando for necessário prover redundância em certos recursos do Kubernetes, poderemos
escalar horizontalmente um ou mais *deployments*, criando *ReplicaSets* para os
mesmos.

Para tanto, use um comando como o mostrado a seguir, definindo o número de
*ReplicaSets*:

```bash
kubectl scale deployment/<nome-do-deployment> --replicas=1
```

Note que introduzir um número de zero réplicas efetivamente eliminará todos os
*pods* do *deployment*, até que as réplicas sejam definidas novamente.
