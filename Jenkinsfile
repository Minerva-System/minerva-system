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
		
                stage('Compilação') {
                    sh 'protoc --version'
                    sh 'cargo build --release'
                }

		stage('Empacotar microsserviços') {
		    sh '''
                        mkdir builds
                        cd builds
                        cp ../target/release/minerva-dispatch ./
                        cp ../target/release/minerva-rest ./
                        cp ../target/release/minerva-runonce ./
                        cp ../target/release/minerva-session ./
                        cp ../target/release/minerva-user ./
                        tar -czvf "../minerva-services.tar.gz" *
                        cd .. && rm -r builds
                    '''
		}

		stage('Empacotar configuração cloud') {
		    sh '''
                        cd deploy/k8s
                        tar -czvf "../../minerva-k8s.tar.gz" *
                        cd ../swarm
                        tar -czvf "../../minerva-swarm.tar.gz" *
                        cd ../../
                    '''
		}
		
		archiveArtifacts artifacts: '*.tar.gz',
		    allowEmptyArchive: false,
		    fingerprint: true,
		    onlyIfSuccessful: true
            }
        }
    }
}
