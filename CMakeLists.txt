cmake_minimum_required(VERSION 3.16)

set(PROJECT_NAME Project_2)

project(${PROJECT_NAME} VERSION 1.0.0)

add_subdirectory("${CMAKE_CURRENT_LIST_DIR}/3rdparty/SFML")

include_directories("${CMAKE_CURRENT_LIST_DIR}/3rdparty/SFML/include")

set(LINK_FLAGS "${LINK_FLAGS} -DSFML_STATIC")

add_executable(${PROJECT_NAME} "src/main.cpp")

target_link_libraries(${PROJECT_NAME} sfml-system sfml-graphics sfml-audio sfml-window sfml-main )
target_link_libraries(${PROJECT_NAME} ${OPENGL_LIBRARIES})

add_custom_command(TARGET ${PROJECT_NAME}  POST_BUILD
                   COMMAND ${CMAKE_COMMAND} -E copy 
                           ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/bin/sfml-audio-3.dll
                           ${CMAKE_CURRENT_BINARY_DIR}/
                   COMMAND ${CMAKE_COMMAND} -E copy 
                           ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/bin/sfml-graphics-3.dll
                           ${CMAKE_CURRENT_BINARY_DIR}/
                   COMMAND ${CMAKE_COMMAND} -E copy 
                           ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/bin/sfml-network-3.dll
                           ${CMAKE_CURRENT_BINARY_DIR}/
                   COMMAND ${CMAKE_COMMAND} -E copy 
                           ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/bin/sfml-system-3.dll
                           ${CMAKE_CURRENT_BINARY_DIR}/
                   COMMAND ${CMAKE_COMMAND} -E copy 
                           ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/bin/sfml-window-3.dll
                           ${CMAKE_CURRENT_BINARY_DIR}/)