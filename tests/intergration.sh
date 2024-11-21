#!/bin/bash
check_requirements() {
    tools=(cargo curl)
    for tool in "${tools[@]}"; do
        if [[ $? -ne 0 ]]; then
            echo "this script require to have $tool installed"
            exit 1
        fi
    done
}

start() {
    cargo run --release 2> /dev/null &
    sleep 1
    pid=$!
}

down() {
    kill $1
}

errors=0
test() {
    local test_name=$1
    local test_function=$2

    start

    echo -n "$test_name : "

    $test_function 2&> /dev/null

    if [[ $? -eq 0 ]]; then
        echo "✅"
    else
        echo "❌"
        errors=$(($errors+1))
    fi

    down $pid
}

check_requirements

login_wrong_pass() {
    curl -f --location 'localhost:8999/api/login' \
    --header 'Content-Type: application/json' \
    --data '{
        "password": "toto"
    }'

    if [[ $? != 0 ]]; then
        return 0
    else
        return 1
    fi
}

login_good_pass() {
    curl -f --location 'localhost:8999/api/login' \
    --header 'Content-Type: application/json' \
    --data '{
        "password": "wake-up!"
    }'

    return $?
}

echo "========================== BUILD =========================="
cargo build --release
echo "========================== TESTS =========================="
test "Login with good password" login_good_pass
test "Login with wrong password" login_wrong_pass
echo "==========================================================="

exit $errors