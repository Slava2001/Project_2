cmake_minimum_required(VERSION 3.16)

set(PROJECT_NAME Project_2)

project(${PROJECT_NAME} VERSION 0.0.0)


set(CMAKE_BUILD_TYPE Release) # Selects the build configuration type such as Debug and .
set(BUILD_SHARED_LIBS static) # Boolean to control whehther dynamic or static libraries are built.

add_subdirectory("${CMAKE_CURRENT_LIST_DIR}/3rdparty/SFML" EXCLUDE_FROM_ALL)
include_directories("${CMAKE_CURRENT_LIST_DIR}/3rdparty/SFML/include")

set(LINK_FLAGS "${LINK_FLAGS} -DSFML_STATIC")

include_directories("${CMAKE_CURRENT_LIST_DIR}/include")
set(SRC "src/main.cpp"
        "src/settings.cpp"
        "src/resources.cpp"
        "src/debug-drawer.cpp")

add_executable(${PROJECT_NAME} ${SRC})

target_link_libraries(${PROJECT_NAME} sfml-system sfml-graphics sfml-audio sfml-window sfml-main )
target_link_libraries(${PROJECT_NAME} ${OPENGL_LIBRARIES})


install(TARGETS ${PROJECT_NAME} RUNTIME DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/bin/sfml-audio-3.dll    DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/bin/sfml-graphics-3.dll DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/bin/sfml-system-3.dll   DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/bin/sfml-window-3.dll   DESTINATION ./)
install(DIRECTORY ${CMAKE_CURRENT_LIST_DIR}/resources DESTINATION ./)
