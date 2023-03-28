
packaging = load 'packaging.groovy'
compilation = load 'compilation.groovy'

podTemplate(containers: [
    containerTemplate(
        name: 'rust',
        image: 'rust:1.65.0',
        command: 'sleep',
        args: '30d'
    )
]) {
    node(POD_LABEL) {
	stage('Instalação de dependências') {
	    container('rust') {
		stage('Instalar Protobuf Compiler v21.7') {
		    sh '''
                        ARCH=`uname -m | sed 's/aarch64/aarch_64/g'`
                        wget https://github.com/protocolbuffers/protobuf/releases/download/v21.7/protoc-21.7-linux-${ARCH}.zip
                        mv protoc-*.zip protoc.zip
                        mkdir protoc && cd protoc
                        unzip ../protoc.zip && rm ../protoc.zip readme.txt
                        mv bin/protoc /usr/local/bin/protoc
                        mv include/* /usr/local/include
                        rm -r bin include
                        cd .. && rm -r protoc
                        protoc --version
                        '''
		}
	    }
	}
	
        stage('Geração de Versão') {
            container('rust') {
                stage('Clonar repositório') {
                    git 'https://github.com/Minerva-System/minerva-system'
                }
		
                compiling.compile_services()
		packaging.package_services()
		packaging.package_config()
            }
        }
    }
}
