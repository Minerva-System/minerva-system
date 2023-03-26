# Testes de integração

## Objetivo

Os testes de integração são criados de forma mais rara, por exigirem um pouco
mais de esforço de implementação, pois geralmente demandam que um ou mais
serviço esteja em execução para que a integração entre as partes do sistema
seja testada.

Atualmente, apenas o módulo `minerva-rest` possui testes de integração. Estes
testes geralmente iniciam outros microsserviços com os quais a aplicação se
comunica.

Um padrão comum no Minerva System é garantir que os testes de integração sejam
executados sequencialmente, e não paralelamente, para evitar concorrência
entre os módulos.

## Execução

Os testes de integração são executados juntamente com os unitários ao usar
o comando `cargo test` na raiz do projeto.
