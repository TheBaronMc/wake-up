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
    sleep 0.2
    pid=$!
}

down() {
    kill $1
}

errors=0
test_index=1
test() {
    local test_name=$1
    local test_function=$2
    local output_file="/tmp/wake_up_test_output_$test_index"

    start

    echo -n "$test_name : "

    $test_function 2&> $output_file 

    if [[ $? -eq 0 ]]; then
        echo "✅"
    else
        echo "❌"
        errors=$(($errors+1))

        echo "-------------------------- OUPUT ------------------------------"
        cat $output_file
        echo "---------------------------------------------------------------"
    fi

    test_index=$(($test_index+1))
    down $pid
}

check_requirements

generate_token() {
    token=$(curl -s --location 'localhost:8999/api/login' \
    --header 'Content-Type: application/json' \
    --data '{
        "password": "wake-up!"
    }' | jq -r '.token')
}

login_wrong_pass() {
    curl -vf --location 'localhost:8999/api/login' \
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
    curl -vf --location 'localhost:8999/api/login' \
    --header 'Content-Type: application/json' \
    --data '{
        "password": "wake-up!"
    }'

    return $?
}

configuration_reload_protected() {
    curl -vf --location 'localhost:8999/api/configuration/reload' 

    if [[ $? != 0 ]]; then
        return 0
    else
        return 1
    fi
}

configuration_reload() {
    generate_token

    curl -vf --location 'localhost:8999/api/configuration/reload' \
        -H "Authorization: Bearer $token"

    return $?
}

wake_up_group_protected() {
    curl -vf --location 'localhost:8999/api/groups/1' 

    if [[ $? != 0 ]]; then
        return 0
    else
        return 1
    fi
}

wake_up_host_group_protected() {
    curl -vf --location 'localhost:8999/api/groups/1/1' 

    if [[ $? != 0 ]]; then
        return 0
    else
        return 1
    fi
}

wake_up_host_protected() {
    curl -vf --location 'localhost:8999/api/hosts/1' 

    if [[ $? != 0 ]]; then
        return 0
    else
        return 1
    fi
}



echo "========================== BUILD =========================="
cargo build --release
echo "========================== TESTS =========================="
test "/login good password" login_good_pass
test "/login wrong password" login_wrong_pass
test "/api/configuration/reload protected" configuration_reload_protected
test "/api/configuration/reload" configuration_reload
test "/api/groups/<group_id> protected" wake_up_group_protected
test "/api/groups/<group_id>/<host_id> protected" wake_up_host_group_protected
test "/api/host/<host_id> protected" wake_up_host_protected
echo "==========================================================="

exit $errors