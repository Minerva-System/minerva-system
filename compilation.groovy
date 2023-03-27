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
