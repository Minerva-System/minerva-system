# Mensageria via RabbitMQ

<center>
<img src="./rabbitmq.png" alt="RabbitMQ" width="200"/>
</center>

O Minerva System utiliza o RabbitMQ para operações de mensageria. Essas
operações fazem-se necessárias sobretudo quando há necessidade de operações
assíncronas, especialmente as que não dependem diretamente de interação do
usuário no sistema, ou que são intensivas em termos de recursos.

## Situações para uso de mensageria

O uso de mensageria deverá ser realizado em operações que devem ser gerenciadas
a partir de eventos, sobretudo quando a operação é intensiva e com eventual
chance de _timeout_. Esses casos incluem, mas não se limitam a:

- Processamento de dados em _batch_;
- Operações mais demoradas (ex. geração de relatório);
- Notificações do usuário (ex. conclusão de processos);
- Operações mais simples, mas que não deverão bloquear o uso do resto
  do sistema (ex. uma operação decorrente de outra).

## Atuais usos no sistema

A seguir, serão listados os atuais usos de mensageria no Minerva System.

### Remoção de sessões ao remover usuário

Ao remover o usuário, para garantir que o mesmo não possa realizar mais nenhuma
operação no sistema, é interessante realizar a remoção de suas sessões. Isso
é feito de forma desvinculada ao processo de remoção do usuário em si, seguindo
o seguinte fluxo:

1. O administrador requisita a remoção de um usuário para o tenant atual.
2. O usuário é removido pelo serviço USER, para o tenant atual.
3. O serviço USER enfileira, na fila específica do tenant para isso, uma
   requisição de remoção de todas as sessões do usuário recentemente removido.
4. O serviço DISPATCH escuta as mensagens na fila do tenant em questão, recebendo
   a mensagem de remoção de sessões.
5. O serviço DISPATCH requisita todas as sessões do usuário removido que estiverem
   na coleção de sessões do tenant atual, no MongoDB.
6. O serviço DISPATCH requisita ao serviço SESSION que cada uma dessas sessões
   seja removida.
7. Para cada sessão, SESSION remove a sessão do MongoDB para o tenant atual, e
   remove a sessão do cache no serviço Redis, caso exista.
8. O serviço DISPATCH dá a mensagem como consumida e retorna à operação de
   escuta para a próxima mensagem.

