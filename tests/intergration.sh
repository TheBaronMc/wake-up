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
    sleep 0.5
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

        echo "========================== BUILD =========================="
        echo "-------------------------- OUPUT --------------------------"
        cat $output_file
        echo "-----------------------------------------------------------"
    fi

    test_index=$(($test_index+1))
    down $pid
}

check_requirements

# UTILS

generate_token() {
    token=$(curl --request POST -s --location 'localhost:8999/api/login' \
    --header 'Content-Type: application/json' \
    --data '{
        "password": "wake-up!"
    }' | jq -r '.token')
}

reload_configuration () {
    generate_token

    curl --request GET -vf --location 'localhost:8999/api/configuration/reload' \
        -H "Authorization: Bearer $token"
}


# TEST FUNCTIONS

login_wrong_pass() {
    curl --request POST -vf --location 'localhost:8999/api/login' \
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
    curl --request POST -vf --location 'localhost:8999/api/login' \
    --header 'Content-Type: application/json' \
    --data '{
        "password": "wake-up!"
    }'

    return $?
}

configuration_reload_protected() {
    curl --request GET -vf --location 'localhost:8999/api/configuration/reload' 

    if [[ $? != 0 ]]; then
        return 0
    else
        return 1
    fi
}

configuration_reload() {
    generate_token

    curl --request GET -vf --location 'localhost:8999/api/configuration/reload' \
        -H "Authorization: Bearer $token"

    return $?
}

wake_up_group_protected() {
    curl --request POST -vf --location 'localhost:8999/api/groups/1' 

    if [[ $? != 0 ]]; then
        return 0
    else
        return 1
    fi
}

wake_up_group_not_found() {
    generate_token

    curl --request POST -vf --location 'localhost:8999/api/groups/1' \
        -H "Authorization: Bearer $token"

    if [[ $? != 0 ]]; then
        return 0
    else
        return 1
    fi
}

wake_up_group() {
    if [[ -f "configuration.yml" ]]; then
        mv configuration.yml configuration.yml.save
    fi

    cat << EOF > configuration.yml
groups:
  groupe1:
    machine1.1: 
      port: 9
      address: 3A:1F:5D:7C:8A:3B 
    machine1.2:
      port: 7
      address: C4:22:5B:0D:9E 
EOF

    reload_configuration

    curl --request POST -vf --location 'localhost:8999/api/groups/groupe1' \
        -H "Authorization: Bearer $token"
    code=$?

    rm configuration.yml

    if [[ -f "configuration.yml.save" ]]; then
        mv configuration.yml.save configuration.yml
    fi

    return $code
}

wake_up_group_host() {
    if [[ -f "configuration.yml" ]]; then
        mv configuration.yml configuration.yml.save
    fi

    cat << EOF > configuration.yml
groups:
  groupe1:
    machine1.1: 
      port: 9
      address: 3A:1F:5D:7C:8A:3B 
    machine1.2:
      port: 7
      address: C4:22:5B:0D:9E 
EOF

    reload_configuration

    curl --request POST -vf --location 'localhost:8999/api/groups/groupe1/machine1.1' \
        -H "Authorization: Bearer $token"
    code=$?

    rm configuration.yml

    if [[ -f "configuration.yml.save" ]]; then
        mv configuration.yml.save configuration.yml
    fi

    return $code
}

wake_up_group_host_not_found() {
    if [[ -f "configuration.yml" ]]; then
        mv configuration.yml configuration.yml.save
    fi

    cat << EOF > configuration.yml
groups:
  groupe1:
    machine1.1: 
      port: 9
      address: 3A:1F:5D:7C:8A:3B
EOF

    reload_configuration

    curl --request POST -vf --location 'localhost:8999/api/groups/groupe1/machine1.2' \
        -H "Authorization: Bearer $token"
    code=$?

    rm configuration.yml

    if [[ -f "configuration.yml.save" ]]; then
        mv configuration.yml.save configuration.yml
    fi

    if [[ $code != 0 ]]; then
        return 0
    else
        return 1
    fi
}

wake_up_host_group_protected() {
    curl --request POST -vf --location 'localhost:8999/api/groups/1/1' 

    if [[ $? != 0 ]]; then
        return 0
    else
        return 1
    fi
}

wake_up_host_protected() {
    curl --request POST -vf --location 'localhost:8999/api/hosts/1' 

    if [[ $? != 0 ]]; then
        return 0
    else
        return 1
    fi
}

wake_up_host_not_found() {
    reload_configuration

    curl --request POST -vf --location 'localhost:8999/api/hosts/host1' \
        -H "Authorization: Bearer $token"
    code=$?

    if [[ $code != 0 ]]; then
        return 0
    else
        return 1
    fi
}

wake_up_host() {
    if [[ -f "configuration.yml" ]]; then
        mv configuration.yml configuration.yml.save
    fi

    cat << EOF > configuration.yml
hosts:
  host1: 
    port: 9
    address: 3A:1F:5D:7C:8A:3B
EOF

    reload_configuration

    curl --request POST -vf --location 'localhost:8999/api/hosts/host1' \
        -H "Authorization: Bearer $token"
    code=$?

    rm configuration.yml

    if [[ -f "configuration.yml.save" ]]; then
        mv configuration.yml.save configuration.yml
    fi

    return $code
}

# RUN TESTS

echo "========================== BUILD =========================="
cargo build --release
echo "========================== TESTS =========================="
test "/login good password" login_good_pass
test "/login wrong password" login_wrong_pass
test "/api/configuration/reload protected" configuration_reload_protected
test "/api/configuration/reload" configuration_reload
test "/api/groups/<group_id> protected" wake_up_group_protected
test "/api/groups/1 not_found" wake_up_group_not_found
test "/api/groups/groupe1" wake_up_group
test "/api/groups/<group_id>/<host_id> protected" wake_up_host_group_protected
test "/api/groups/groupe1/machine1.2 not_found" wake_up_group_host_not_found
test "/api/groups/groupe1/machine1.1" wake_up_group_host
test "/api/host/<host_id> protected" wake_up_host_protected
test "/api/host/host1 not_found" wake_up_host_not_found
test "/api/host/host1" wake_up_host
echo "==========================================================="

exit $errors