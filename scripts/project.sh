#!/bin/bash

mode=""
service=""
action=""

help() {
    echo "Usage:"
    echo "  --local  (-L)      : Start services locally"
    echo "  --docker (-D)      : Start services in Docker"
    echo "  --{{name_service}} : Specify a specific service (optional)"
    echo "  --run    (-R)      : Run the service(s)"
    echo "  --build  (-B)      : Build the service(s)"
    echo "  --tests            : Run tests"
    echo "  --test --init      : Initialize environment and run tests"
    echo "  --help             : Show this message"
}

run_tests() {
    cd ../testing || exit
    docker-compose -f docker-compose.test.yaml up --build -d
    export AD_ENGINE_ADDRESS=localhost:8000
    source .venv/bin/activate
    sleep 5
    pytest -v --tavern-global-cfg=tavern.config.yaml  
    docker-compose -f docker-compose.test.yaml down -v
    cd ../microservices/ad_engine || exit
    cargo test
    exit
}

init_and_run_tests() {
    echo "Creating virtual environment..."
    cd ../testing || exit
    python -m venv .venv
    source .venv/bin/activate
    echo "Installing dependencies..."
    pip install -r requirements.txt
    cd ../scripts || exit
    echo "Running tests..."
    run_tests
}

parse_args() {
    for arg in "$@"; do
        case "$arg" in
            --e2e)
                run_tests
                ;;
            --test)
                mode="test"
                ;;
            --init)
                action="init"
                ;;
            --local | -L)
                mode="local"
                ;;
            --docker | -D)
                mode="docker"
                ;;
            --help)
                help
                exit
                ;;
            --run | -R)
                action="run"
                ;;
            --build | -B)
                action="build"
                ;;
            *)
                service="$arg"
                echo "service: $arg"
                ;;
        esac
    done

    # Check if both --test and --init are present
    if [[ "$mode" == "test" && "$action" == "init" ]]; then
        init_and_run_tests
    fi
}

validate_args() {
    if [[ -z "$mode" ]]; then
        echo "--local or --docker flag not specified."
        exit 1
    fi
    if [[ -z "$action" ]]; then
        echo "--run or --build flag not specified."
        exit 1
    fi
}

execute_commands() {
    case "$mode" in
        local)
            manage_local_service
            ;;
        docker)
            manage_docker_service
            ;;
    esac
}

manage_local_service() {
    if [[ -z "$service" ]]; then
        echo "Starting all services with command $action..."
        start_all_local_services
    else
        echo "Starting service $service with command $action..."
        start_specific_local_service "$service"
    fi
}

start_all_local_services() {
    pg_ctl restart
    cd ../microservices/ad_engine/ || exit
    cargo "$action"
    echo "Done ad_engine!"

    cd ../telegram_bot/ || exit
    poetry install
    poetry run bot
    echo "Done telegram_bot!"
}

start_specific_local_service() {
    local current_service="$1"
    if [[ "$current_service" == "ad_engine" ]]; then
        pg_ctl restart
        cd ../microservices/ad_engine/ || exit
        cargo "$action"
        echo "Done!"
    elif [[ "$current_service" == "telegram_bot" ]]; then
        cd ../microservices/telegram_bot/ || exit
        if [[ "$action" == "build" ]]; then
            poetry install
            echo "Done!"
        else
            poetry run python main.py
            echo "Done!"
        fi
    fi
}

manage_docker_service() {
    echo "Starting docker-compose with command $action..."
    cd ../deploy/ || exit
    docker_commands "$service" "$action"
}

docker_commands() {
    local current_service="$1"
    local current_action="$2"
    local compose_file="docker-compose.dev.yaml"

    case "$current_service" in
        ad_engine)
            if [[ "$current_action" == "build" ]]; then
                docker-compose -f "$compose_file" up rust_ad_engine --build
            else
                docker-compose -f "$compose_file" up rust_ad_engine
            fi
            ;;
        telegram_bot)
            if [[ "$current_action" == "build" ]]; then
                docker-compose -f "$compose_file" up python_telegram_bot --build
            else
                docker-compose -f "$compose_file" up python_telegram_bot
            fi
            ;;
        *)
            if [[ "$current_action" == "build" ]]; then
                docker-compose -f "$compose_file" up --build
            else
                docker-compose -f "$compose_file" up
            fi
            ;;
    esac
}

parse_args "$@"
validate_args
execute_commands
