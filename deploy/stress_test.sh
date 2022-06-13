#!/bin/bash

function usage() {
    echo "Uso: ./stress_test.sh <endpoint> <tipo>"
    echo "Onde <endpoint> é o caminho para a API (ex. minerva-system.io/api)"
    echo "e <tipo> é um dos possíveis sistemas:"
    echo "  - users"
    echo "  - session"
    echo
    echo "Exemplo:"
    echo "   ./stress_test.sh minerva-system.io/api users"
    echo
}

if [ "$#" -ne 2 ]; then
    usage
    exit 1
fi

SERVER=$1
TEST_TYPE=$2

TENANT=teste
SESSION_STRESS_LOGIN_URL="http://${SERVER}/${TENANT}/login"
SESSION_STRESS_LOGOUT_URL="http://${SERVER}/logout"
USER_STRESS_URL="http://${SERVER}/users/1"
COOKIE_FILE=/tmp/minerva_cookie.txt


function get_session_cookie() {
    echo "Logging in...";
    curl -s -X POST $SESSION_STRESS_LOGIN_URL \
	 -H 'Content-Type: application/json' \
	 -d '{"login": "admin", "password": "admin"}' \
	 -c $COOKIE_FILE;
    echo
}

function remove_session_cookie() {
    echo "Logging out...";
    curl -s -X POST $SESSION_STRESS_LOGOUT_URL \
	     -b $COOKIE_FILE;
    rm -f $COOKIE_FILE;
    echo
}


function users_stress_test() {
    get_session_cookie;
    for i in {1..10000}; do
	curl -s -X GET $USER_STRESS_URL \
	     -b $COOKIE_FILE >> /dev/null;
	sleep "0.001";
    done;
    remove_session_cookie;
}

function session_stress_test() {
    mkdir -p /tmp/minerva_cookies/
    # Ninguém faz tanto login ou logout assim. Certo?...
    sleeptime=0.01
    
    for i in {1..10000}; do
	cookiefile="/tmp/minerva_cookies/$(cat /dev/urandom | tr -cd 'a-f0-9' | head -c 32).txt";
	curl -s -X POST $SESSION_STRESS_LOGIN_URL \
	     -H 'Content-Type: application/json' \
	     -d '{"login": "admin", "password": "admin"}' \
	     -c $cookiefile >> /dev/null
	sleep 0.01
	
	curl -s -X POST $SESSION_STRESS_LOGOUT_URL \
	     -b $cookiefile >> /dev/null
	sleep 0.01
    done;
    
    rm -r /tmp/minerva-cookies
}




echo 'Iniciando teste de stress. Use Ctrl+C para encerrar.'
case $TEST_TYPE in
    "users")
	users_stress_test
    ;;

    "session")
	session_stress_test
    ;;

    *)
	echo "Sistema ${TEST_TYPE} desconhecido."
	echo
	usage
	exit 1
    ;;
esac
