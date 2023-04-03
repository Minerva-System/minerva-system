#!/usr/bin/env groovy

def call() {
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
