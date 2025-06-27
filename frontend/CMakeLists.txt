cmake_minimum_required(VERSION 3.16)
project(AIAntivirusFrontend)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

find_package(Qt6 REQUIRED COMPONENTS Widgets Network Svg WebSockets)

add_executable(AIAntivirus WIN32
    resources/resources.qrc
    
    src/main.cpp
    src/mainwindow.h
    src/mainwindow.cpp
    
    src/api/api_client.h
    src/api/api_client.cpp
    
    src/widgets/navigationbar.h
    src/widgets/navigationbar.cpp
    src/widgets/shield_widget.h
    src/widgets/shield_widget.cpp
    src/widgets/status_card_widget.h
    src/widgets/status_card_widget.cpp
    src/widgets/futuristic_dropzone.h
    src/widgets/futuristic_dropzone.cpp

    src/pages/dashboard_page.h
    src/pages/dashboard_page.cpp
    src/pages/scan_page.h
    src/pages/scan_page.cpp
    src/pages/quarantine_page.h
    src/pages/quarantine_page.cpp
    src/pages/settings_page.h
    src/pages/settings_page.cpp
    src/pages/protection_page.h
    src/pages/protection_page.cpp
    
    src/delegates/quarantine_item_delegate.h
    src/delegates/quarantine_item_delegate.cpp
)

target_link_libraries(AIAntivirus PRIVATE Qt6::Widgets Qt6::Network Qt6::Svg Qt6::WebSockets)

if(MSVC)
    target_compile_options(AIAntivirus PRIVATE /W4 /Zc:__cplusplus /permissive-)
endif()