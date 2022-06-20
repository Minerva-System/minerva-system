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

package USER {
	usecase cadastro
	usecase listagem
	usecase consulta
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
actor :Administrador do Sistema: as sysadmin

package TENANCY {
	usecase "listagem de nomes" as listagem_nomes
	usecase listagem
	usecase cadastro
	usecase desativação as desativacao
	
	note right of listagem_nomes
		Esta funcionalidade não requer autenticação,
		pois poderá ser utilizada na tela de login.
	end note
}

user -- listagem_nomes
sysadmin -- listagem
sysadmin -- cadastro
sysadmin -- desativacao

note top of sysadmin
	O administrador do sistema deve ter permissões
	de alterações globais apenas para gerenciamento
	de inquilinos.
end note

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
	usecase "obter dados do relatório" as obter_dados
	usecase "emitir PDF do relatório" as emitir_pdf
	
	note "O usuário deverá ter iniciado\numa sessão para estes casos." as must_have_session
	
	(obter_dados)..must_have_session
	(emitir_pdf)..must_have_session
}

user -- obter_dados
user -- emitir_pdf

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
	usecase "início de estoque" as inicio
	usecase movimentação as movimentacao
	usecase entrada as entrada
	usecase saída as saida
	usecase consulta as consulta
	usecase "listagem de estoques" as listagem
	usecase "listagem de movimentações" as listagem_movimentos

	(movimentacao) .> (inicio) : << extend >>
	(entrada) .> (movimentacao) : << include >>
	(saida) .> (movimentacao) : << include >>
	
	note "O usuário deverá ter iniciado\numa sessão para estes casos." as must_have_session
	note "Para validar esses casos,\no produto associado deve existir." as product_must_exist
	
	(inicio)..must_have_session
	(entrada)..must_have_session
	(saida)..must_have_session
	(consulta)..must_have_session
	(listagem)..must_have_session
	(listagem_movimentos)..must_have_session
	
	(inicio)..product_must_exist
	(movimentacao)..product_must_exist
	(consulta)..product_must_exist
	(listagem_movimentos)..product_must_exist
}

user -- inicio
user -- entrada
user -- saida
user -- consulta
user -- listagem
user -- listagem_movimentos

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
	usecase "enviar mensagem instantânea" as enviar
	usecase "enviar para WhatsApp" as whatsapp
	usecase "enviar para Facebook Messenger" as messenger
	usecase "enviar para Instagram" as instagram
	usecase "enviar para Telegram" as telegram
	
	note "O usuário deverá ter iniciado\numa sessão para estes casos." as must_have_session
	
	(whatsapp) .> (enviar) : << include >>
	(messenger) .> (enviar) : << include >>
	(instagram) .> (enviar) : << include >>
	(telegram) .> (enviar) : << include >>
	
	(whatsapp)..must_have_session
	(messenger)..must_have_session
	(instagram)..must_have_session
	(telegram)..must_have_session
	
	note right of enviar
		Os dados de envio da plataforma devem
		existir no cadastro do cliente
		referenciado.
	end note
}

user -- whatsapp
user -- messenger
user -- instagram
user -- telegram

@enduml
```

</center>


