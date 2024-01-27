cmake_minimum_required(VERSION 3.16)

set(PROJECT_NAME Project_2)

project(${PROJECT_NAME} VERSION 0.0.0)

if(NOT DEFINED _DISABLE_INSTALLATION)
  set(_DISABLE_INSTALLATION FALSE)
  macro(install)
    if (NOT _DISABLE_INSTALLATION)
      _install(${ARGN})
    endif()
  endmacro()
endif()

set(CMAKE_BUILD_TYPE Release)
set(BUILD_SHARED_LIBS static)

set(SFML_BUILD_EXAMPLES FALSE)
set(SFML_BUILD_DOC FALSE)
set(SFML_INSTALL_PKGCONFIG_FILES FALSE)
set(_DISABLE_INSTALLATION TRUE)
add_subdirectory("${CMAKE_CURRENT_LIST_DIR}/3rdparty/SFML")
include_directories("${CMAKE_CURRENT_LIST_DIR}/3rdparty/SFML/include")
set(_DISABLE_INSTALLATION FALSE)

add_subdirectory("${CMAKE_CURRENT_LIST_DIR}/3rdparty/json")
include_directories("${CMAKE_CURRENT_LIST_DIR}/3rdparty/json/include")

set(LINK_FLAGS "${LINK_FLAGS} -DSFML_STATIC")

include_directories("${CMAKE_CURRENT_LIST_DIR}/include/lan")
set(LAN_SRC "src/lan/lan-server.cpp")
add_library(LAN_LIB STATIC ${LAN_SRC})

include_directories("${CMAKE_CURRENT_LIST_DIR}/include/scene")
set(SCENE_SRC "src/scene/scene-manager.cpp"
              "src/scene/scene-base.cpp"
              "src/scene/scene-debug.cpp"
              "src/scene/scene-test-server.cpp")
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

target_link_libraries(LAN_LIB sfml-network)
target_link_libraries(${PROJECT_NAME} LAN_LIB)
target_link_libraries(SCENE_LIB LAN_LIB)
target_link_libraries(${PROJECT_NAME} SCENE_LIB)
target_link_libraries(${PROJECT_NAME} GUI_LIB)
target_link_libraries(${PROJECT_NAME} sfml-system sfml-graphics sfml-audio)
if (WIN32)
    target_link_libraries(${PROJECT_NAME} sfml-window sfml-main)
endif (WIN32)
target_link_libraries(${PROJECT_NAME} ${OPENGL_LIBRARIES})

install(TARGETS ${PROJECT_NAME} RUNTIME DESTINATION ./)
if (WIN32)
    install(DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/bin/
            DESTINATION ./
            FILES_MATCHING PATTERN "*.dll")
elseif (UNIX)
    install(DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/3rdparty/SFML/lib/
            DESTINATION ./
            FILES_MATCHING PATTERN "*.so.*")
endif (WIN32)
install(DIRECTORY ${CMAKE_CURRENT_LIST_DIR}/resources DESTINATION ./)
