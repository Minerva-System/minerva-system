#!/usr/bin/env groovy

def call() {
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
