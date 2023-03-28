def package_services() {
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
	archiveArtifacts artifacts: 'minerva-services.tar.gz',
	    allowEmptyArchive: false,
	    fingerprint: true,
	    onlyIfSuccessful: true
    }
}

def package_config() {
    stage('Empacotar configuração cloud') {
	sh '''
                        cd deploy/k8s
                        tar -czvf "../../minerva-k8s.tar.gz" *
                        cd ../swarm
                        tar -czvf "../../minerva-swarm.tar.gz" *
                        cd ../../
                    '''
	archiveArtifacts artifacts: 'minerva-k8s.tar.gz',
	    allowEmptyArchive: false,
	    fingerprint: true,
	    onlyIfSuccessful: true

	archiveArtifacts artifacts: 'minerva-swarm.tar.gz',
	    allowEmptyArchive: false,
	    fingerprint: true,
	    onlyIfSuccessful: true
    }
}

return this;
