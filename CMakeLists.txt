cmake_minimum_required(VERSION 3.16)

set(PROJECT_NAME Project_2)

project(${PROJECT_NAME} VERSION 0.0.0)

set(CMAKE_BUILD_TYPE Release) # Selects the build configuration type such as Debug and .
set(BUILD_SHARED_LIBS static) # Boolean to control whehther dynamic or static libraries are built.

add_subdirectory("${CMAKE_CURRENT_LIST_DIR}/3rdparty/SFML" EXCLUDE_FROM_ALL)
include_directories("${CMAKE_CURRENT_LIST_DIR}/3rdparty/SFML/include")

add_subdirectory("${CMAKE_CURRENT_LIST_DIR}/3rdparty/json")
include_directories("${CMAKE_CURRENT_LIST_DIR}/3rdparty/json/include")

set(LINK_FLAGS "${LINK_FLAGS} -DSFML_STATIC")

include_directories("${CMAKE_CURRENT_LIST_DIR}/include/scene")
set(SCENE_SRC "src/scene/scene-manager.cpp"
              "src/scene/scene-base.cpp"
              "src/scene/scene-debug.cpp")
add_library(SCENE_LIB STATIC ${SCENE_SRC})

include_directories("${CMAKE_CURRENT_LIST_DIR}/include/gui")
set(GUI_SRC "src/gui/gui-manager.cpp"
            "src/gui/gui-base.cpp"
            "src/gui/gui-panel.cpp"
            "src/gui/gui-textbox.cpp"
            "src/gui/gui-button.cpp"
            "src/gui/gui-slider.cpp")
add_library(GUI_LIB STATIC ${GUI_SRC})

include_directories("${CMAKE_CURRENT_LIST_DIR}/include")
set(SRC "src/main.cpp"
        "src/settings.cpp"
        "src/resources.cpp"
        "src/debug-drawer.cpp"
        "src/util.cpp")

SET(CMAKE_CXX_FLAGS  "${CMAKE_CXX_FLAGS} -Wall -Werror -Wextra -Wno-unused-parameter")
add_executable(${PROJECT_NAME} ${SRC})

target_link_libraries(${PROJECT_NAME} SCENE_LIB)
target_link_libraries(${PROJECT_NAME} GUI_LIB)
target_link_libraries(${PROJECT_NAME} sfml-system sfml-graphics sfml-audio)
if (WIN32)
target_link_libraries(${PROJECT_NAME} sfml-window sfml-main)
endif (WIN32)
target_link_libraries(${PROJECT_NAME} ${OPENGL_LIBRARIES})

install(TARGETS ${PROJECT_NAME} RUNTIME DESTINATION ./)
if (WIN32)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/bin/sfml-audio-3.dll    DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/bin/sfml-graphics-3.dll DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/bin/sfml-system-3.dll   DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/bin/sfml-window-3.dll   DESTINATION ./)
elseif (UNIX)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/lib/libsfml-audio.so          DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/lib/libsfml-audio.so.3.0      DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/lib/libsfml-audio.so.3.0.0    DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/lib/libsfml-graphics.so       DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/lib/libsfml-graphics.so.3.0   DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/lib/libsfml-graphics.so.3.0.0 DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/lib/libsfml-system.so         DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/lib/libsfml-system.so.3.0     DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/lib/libsfml-system.so.3.0.0   DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/lib/libsfml-window.so         DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/lib/libsfml-window.so.3.0     DESTINATION ./)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/lib/libsfml-window.so.3.0.0   DESTINATION ./)
endif (WIN32)
install(DIRECTORY ${CMAKE_CURRENT_LIST_DIR}/resources DESTINATION ./)
