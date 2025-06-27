cmake_minimum_required(VERSION 3.15)
project(system_integration CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Kütüphaneyi oluştur
add_library(system_integration SHARED 
    src/windows/file_watcher.cpp 
    # src/linux/file_watcher.cpp # Linux için bu satır aktif edilir
)

# Arayüz başlık dosyalarının yolunu belirt
target_include_directories(system_integration PUBLIC 
    "${CMAKE_CURRENT_SOURCE_DIR}/include"
)

# Windows'a özel ayarlar
if(WIN32)
    target_compile_definitions(system_integration PRIVATE UNICODE _UNICODE)
endif()