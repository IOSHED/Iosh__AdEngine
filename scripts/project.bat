@echo off
setlocal enabledelayedexpansion

REM Initialize variables
set "mode="
set "service="
set "action="

REM Main entry point
call :parse_args %*

REM Validate arguments
call :validate_args

REM Execute based on mode
call :execute_commands

:end
exit /B

:parse_args
for %%A in (%*) do (
    if "%%~A"=="--tests" (
        call :run_tests
        exit /B
    ) else if "%%~A"=="--test" (
        set "mode=test"
    ) else if "%%~A"=="--init" (
        set "action=init"
    ) else if "%%~A"=="--local" (
        set "mode=local"
    ) else if "%%~A"=="-L" (
        set "mode=local"
    ) else if "%%~A"=="--docker" (
        set "mode=docker"
    ) else if "%%~A"=="-D" (
        set "mode=docker"
    ) else if "%%~A"=="--help" (
        call :help
        exit /B
    ) else if "%%~A"=="--run" (
        set "action=run"
    ) else if "%%~A"=="-R" (
        set "action=run"
    ) else if "%%~A"=="--build" (
        set "action=build"
    ) else if "%%~A"=="-B" (
        set "action=build"
    ) else (
        set "service=%%~A"
        echo service: %%~A
    )
)

REM Check if both --test and --init are present
if "%mode%"=="test" if "%action%"=="init" (
    call :init_and_run_tests
    exit /B
)

exit /B

:validate_args
if "%mode%"=="" (
    echo --local or --docker flag not specified.
    exit /B 1
)

if "%action%"=="" (
    echo --run or --build flag not specified.
    exit /B 1
)
exit /B

:execute_commands
if "%mode%"=="local" (
    call :manage_local_service
) else if "%mode%"=="docker" (
    call :manage_docker_service
)
exit /B

:manage_local_service
if "%service%"=="" (
    echo Starting all services with command %action%...
    call :start_all_local_services
) else (
    echo Starting service %service% with command %action%...
    call :start_specific_local_service %service%
)
exit /B

:start_all_local_services
pg_ctl restart
cd ../microservices/ad_engine/
cargo %action%
echo Done ad_engine!

cd ../telegram_bot/
poetry install
poetry run bot
echo Done telegram_bot!
exit /B

:start_specific_local_service
set "current_service=%~1"
if "%current_service%"=="ad_engine" (
    pg_ctl restart
    cd ../microservices/ad_engine/
    cargo %action%
    echo Done!
) else if "%current_service%"=="telegram_bot" (
    cd ../microservices/telegram_bot/
    if "%action%"=="build" (
        poetry install
        echo Done!
    ) else (
        poetry run python main.py
        echo Done!
    )
)
exit /B

:manage_docker_service
echo Starting docker-compose with command %action%...
cd ../deploy/
call :docker_commands %service% %action%
exit /B

:docker_commands
set "current_service=%~1"
set "current_action=%~2"
set "compose_file=docker-compose.dev.yaml"

if "%current_service%"=="ad_engine" (
    if "%current_action%"=="build" (
        docker-compose -f %compose_file% up rust_ad_engine --build
    ) else (
        docker-compose -f %compose_file% up rust_ad_engine
    )
) else if "%current_service%"=="telegram_bot" (
    if "%current_action%"=="build" (
        docker-compose -f %compose_file% up python_telegram_bot --build
    ) else (
        docker-compose -f %compose_file% up python_telegram_bot
    )
) else (
    if "%current_action%"=="build" (
        docker-compose -f %compose_file% up --build
    ) else (
        docker-compose -f %compose_file% up
    )
)
exit /B

:run_tests
cd ../testing
docker-compose -f docker-compose.test.yaml up -d
set AD_ENGINE_ADDRESS=localhost:9000
call .venv\Scripts\activate
timeout /t 5
pytest -v --tavern-global-cfg=tavern.config.yaml  
docker-compose -f docker-compose.test.yaml down -v
cd ../microservices/ad_engine
cargo test
exit /B

:init_and_run_tests
echo Creating virtual environment...
cd ../testing
python -m venv .venv
call .venv\Scripts\activate
echo Installing dependencies...
pip install -r requirements.txt
cd ../scripts
echo Running tests...
call :run_tests
exit /B

:help
echo Usage:
echo   --local  (-L)      : Start services locally
echo   --docker (-D)      : Start services in Docker
echo   --{{name_service}} : Specify a specific service (optional)
echo   --run    (-R)      : Run the service(s)
echo   --build  (-B)      : Build the service(s)
echo   --tests            : Run tests
echo   --test --init      : Initialize environment and run tests
echo   --help             : Show this message
exit /B