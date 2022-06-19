# Diagramas de Casos de Uso

Os diagramas a seguir representam casos de uso para o sistema. Esses diagramas
não têm a pretensão de serem completos, mas sim de ilustrar funcionalidades
esperadas para o sistema, de forma visual.

Os casos de uso foram subdivididos em domínios, que poderão ilustrar os
microsserviços envolvidos.

## Sessão

<center>

```plantuml
@startuml
!theme amiga
left to right direction
actor :Usuário do Sistema: as user

package SESSION {
	usecase login as login
	usecase logoff as logoff
	
	note right of logoff
		O usuário deverá ter iniciado
		uma sessão anteriormente para sair.
	end note
}

user -- login
user -- logoff

@enduml
```

</center>

## Usuários

<center>

```plantuml
@startuml
!theme amiga
left to right direction
actor :Usuário do Sistema: as user

package USERS {
	usecase cadastro as cadastro
	usecase listagem as listagem
	usecase consulta as consulta
	usecase alteração as alteracao
	usecase remoção as remocao
	
	note "O usuário deverá ter iniciado\numa sessão para estes casos." as must_have_session
	
	(cadastro)..must_have_session
	(listagem)..must_have_session
	(consulta)..must_have_session
	(alteracao)..must_have_session
	(remocao)..must_have_session
}

user -- cadastro
user -- listagem
user -- consulta
user -- alteracao
user -- remocao

@enduml
```

</center>

## Inquilinos

<center>

```plantuml
@startuml
!theme amiga
left to right direction
actor :Usuário do Sistema: as user

package TENANCY {
}

@enduml
```

</center>

## Auditoria

<center>

```plantuml
@startuml
!theme amiga
left to right direction
actor :Usuário do Sistema: as user

package AUDIT {
	usecase "consultar logs" as auditoria_consultalogs
	
	note "O usuário deverá ter iniciado\numa sessão para estes casos." as must_have_session
	
	note right of auditoria_consultalogs
		O usuário deve ter permissão
		para consultar os logs.
	end note
	
	(auditoria_consultalogs)..must_have_session
}

user -- auditoria_consultalogs

@enduml
```

</center>

## Relatórios

<center>

```plantuml
@startuml
!theme amiga
left to right direction
actor :Usuário do Sistema: as user

package REPORT {
}

@enduml
```

</center>

## Produtos

<center>

```plantuml
@startuml
!theme amiga
left to right direction
actor :Usuário do Sistema: as user

package PRODUCTS {
	usecase cadastro as cadastro
	usecase listagem as listagem
	usecase consulta as consulta
	usecase alteração as alteracao
	usecase remoção as remocao
	
	note "O usuário deverá ter iniciado\numa sessão para estes casos." as must_have_session
	
	(cadastro)..must_have_session
	(listagem)..must_have_session
	(consulta)..must_have_session
	(alteracao)..must_have_session
	(remocao)..must_have_session
}

user -- cadastro
user -- listagem
user -- consulta
user -- alteracao
user -- remocao


@enduml
```

</center>

## Estoque

<center>

```plantuml
@startuml
!theme amiga
left to right direction
actor :Usuário do Sistema: as user

package STOCK {
	usecase "início de estoque" as estoque_inicio
	usecase movimentação as estoque_movimentacao
	usecase entrada as estoque_entrada
	usecase saída as estoque_saida
	usecase consulta as estoque_consulta

	(estoque_movimentacao) .> (estoque_inicio) : << extend >>
	(estoque_entrada) .> (estoque_movimentacao) : << include >>
	(estoque_saida) .> (estoque_movimentacao) : << include >>
	
	note "O usuário deverá ter iniciado\numa sessão para estes casos." as must_have_session
	note "Para validar esses casos,\no produto associado deve existir." as product_must_exist
	
	(estoque_inicio)..must_have_session
	(estoque_entrada)..must_have_session
	(estoque_saida)..must_have_session
	(estoque_consulta)..must_have_session
	
	(estoque_inicio)..product_must_exist
	(estoque_movimentacao)..product_must_exist
	(estoque_consulta)..product_must_exist
}

user -- estoque_inicio
user -- estoque_entrada
user -- estoque_saida
user -- estoque_consulta

@enduml
```

</center>

## Clientes

<center>

```plantuml
@startuml
!theme amiga
left to right direction
actor :Usuário do Sistema: as user

package CLIENT {
	usecase cadastro as cadastro
	usecase listagem as listagem
	usecase consulta as consulta
	usecase alteração as alteracao
	usecase remoção as remocao
	
	note "O usuário deverá ter iniciado\numa sessão para estes casos." as must_have_session
	
	(cadastro)..must_have_session
	(listagem)..must_have_session
	(consulta)..must_have_session
	(alteracao)..must_have_session
	(remocao)..must_have_session
}

user -- cadastro
user -- listagem
user -- consulta
user -- alteracao
user -- remocao

@enduml
```

</center>


## Comunicação Instantânea

<center>

```plantuml
@startuml
!theme amiga
left to right direction
actor :Usuário do Sistema: as user

package COMM {
}

@enduml
```

</center>


