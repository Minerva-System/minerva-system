# Testes de Carga

<center>
<img src="./k6.png" alt="Grafana k6" height="150"/>
</center>

A aplicação possui também testes de carga para avaliar como o sistema age
sob um grande volume de acessos e operações. Para isso, usamos a ferramenta
[Grafana k6](https://k6.io/).

## Requisitos

Para executar os testes, você precisará do k6 instalado e também do Node
v16.19.1.


## Preparando os testes

Os testes encontram-se no diretório `deploy/tests/k6/load-test`. Navegue
até o mesmo, e então instale as dependências:

```bash
npm install
```

Em seguida, você poderá, opcionalmente, definir algumas variáveis de ambiente
para que o teste funcione, de acordo com a tabela a seguir:

| Variável         | Descrição                         | Valor Padrão                |
|------------------|-----------------------------------|-----------------------------|
| `MINERVA_HOST`   | Host da API do Minerva            | `http://localhost:9000/api` |
| `MINERVA_TENANT` | Inquilino a ser usado para testes | `teste`                     |


## Executando os testes

Em seguida, execute os testes usado o comando:

```bash
npm run test
```

