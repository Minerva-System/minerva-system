podTemplate(containers: [
    containerTemplate(
        name: 'rust',
        image: 'rust:1.65.0',
        command: 'sleep',
        args: '30d'
    )
]) {
    node(POD_LABEL) {
        stage('Geração de Versão') {
            container('rust') {
                stage('Clonar repositório') {
                    git 'https://github.com/Minerva-System/minerva-system'
                }
		
                stage('Compilação') {
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
