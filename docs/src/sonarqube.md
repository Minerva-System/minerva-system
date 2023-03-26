# SonarQube e Quality Gates

O projeto Minerva System atualmente está [hospedado no GitHub](https://github.com/Minerva-System/minerva-system) e, por isso,
utiliza as ferramentas de pipelines e CI/CD do mesmo para execução de testes e
geração de builds.

Para garantir a qualidade no processo de entrega de cada pull request, foi
adicionado suporte a SonarQube, principalmente através do arquivo
`sonar-project.properties` que se encontra na raiz desse projeto.

O GitHub possui acesso ao SonarQube, que está em execução em infraestrutura
própria que, por enquanto, trata-se de um servidor on-premise em um Raspberry Pi
4, mais especificamente executando K3s (uma implementação de Kubernetes). Este
servidor também opera atualmente como o ambiente de testes do Minerva System,
uma vez que o mesmo não possui um ambiente de QA ou produção.

Por isso, pelo menos por enquanto, o projeto não possui um portal de métricas
explícito que possa ser acompanhado por desenvolvedores.

Para que um pull request passe nos quality gates do projeto, ele deverá garantir
que o código possua as seguintes estatísticas:

| Estatística           | Valor para falha |
|-----------------------|------------------|
| Cobertura de testes   | Menor que 50%*   |
| Linhas duplicadas (%) | Maior que 3%*    |
| Code Smells           | Maior que 0      |
| Vulnerabilidades      | Maior que 0      |


As estatísticas marcadas com `*` estão sujeitas a mudança no futuro. Idealmente,
o código deverá ter uma cobertura de no mínimo 80% e a quantidade de linhas
duplicadas poderá ser diminuída após mais análise.
