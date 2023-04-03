#!/usr/bin/env groovy

def call() {
    stage('Compilação') {
        sh 'cargo build --release'
    }
}
