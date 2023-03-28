def compile_services() {
    stage('Compilação') {
        sh 'cargo build --release'
    }
}
