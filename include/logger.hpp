#ifndef INCLUDE_LOGGER_HPP
#define INCLUDE_LOGGER_HPP

#include <iostream>
#include <sstream>
#include <ctime>

#define LOG_LVL_DEBUG 0
#define LOG_LVL_INFO  1
#define LOG_LVL_WARN  2
#define LOG_LVL_ERROR 3
#define LOG_LVL_FATAL 4

#ifndef LOG_LVL
#define LOG_LVL LOG_LVL_INFO
#endif

#define UNWRAP_0(a)
#define UNWRAP_1(a, ...) << a UNWRAP_0(__VA_ARGS__)
#define UNWRAP_2(a, ...) << a UNWRAP_1(__VA_ARGS__)
#define UNWRAP_3(a, ...) << a UNWRAP_2(__VA_ARGS__)
#define UNWRAP_4(a, ...) << a UNWRAP_3(__VA_ARGS__)
#define UNWRAP_5(a, ...) << a UNWRAP_4(__VA_ARGS__)
#define UNWRAP_6(a, ...) << a UNWRAP_5(__VA_ARGS__)
#define UNWRAP_7(a, ...) << a UNWRAP_6(__VA_ARGS__)
#define UNWRAP_8(a, ...) << a UNWRAP_7(__VA_ARGS__)
#define UNWRAP_9(a, ...) << a UNWRAP_8(__VA_ARGS__)
#define UNWRAP_10(a, ...) << a UNWRAP_9(__VA_ARGS__)
#define UNWRAP_NAME(dummy, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, N, ...) UNWRAP ## N
#define UNWRAP_N(...) UNWRAP_NAME(dummy, ##__VA_ARGS__, _10, _9, _8, _7, _6, _5, _4, _3, _2, _1, _0)
#define UNWRAP(...) UNWRAP_N(__VA_ARGS__)(__VA_ARGS__)

#define LOG_COLOR_GRAY   "\033[30m"
#define LOG_COLOR_REG    "\033[31m"
#define LOG_BG_COLOR_REG "\033[41m"
#define LOG_BG_GREEN     "\033[32m"
#define LOG_COLOR_YELLOW "\033[33m"
#define LOG_COLOR_BLUE   "\033[36m"
#define LOG_COLOR_CLEAR  "\033[0m"

/// @brief Log the message with the specified logging level
/// @param lvl logging level
/// @param lvl_name string indicating the logging level
/// @param ... message
#define LOG(lvl, lvl_name, ...)                                          \
    do {                                                                 \
        if (lvl >= LOG_LVL) {                                            \
            std::stringstream msg;                                       \
            msg << "[" << time(0) << "] "                                \
                << lvl_name << " "                                       \
                << LOG_COLOR_GRAY << __func__ << ": " << LOG_COLOR_CLEAR \
                UNWRAP(__VA_ARGS__)                                      \
                << std::endl;                                            \
            std::cout << msg.str();                                      \
        }                                                                \
    } while(0)

#define log_debug(...) LOG(LOG_LVL_DEBUG, LOG_BG_GREEN     "DEBUG" LOG_COLOR_CLEAR, ##__VA_ARGS__)
#define log_info(...)  LOG(LOG_LVL_INFO,  LOG_COLOR_BLUE   "INFO"  LOG_COLOR_CLEAR, ##__VA_ARGS__)
#define log_warn(...)  LOG(LOG_LVL_WARN,  LOG_COLOR_YELLOW "WARN"  LOG_COLOR_CLEAR, ##__VA_ARGS__)
#define log_error(...) LOG(LOG_LVL_ERROR, LOG_COLOR_REG    "ERROR" LOG_COLOR_CLEAR, ##__VA_ARGS__)
#define log_fatal(...) LOG(LOG_LVL_FATAL, LOG_BG_COLOR_REG "FATAL" LOG_COLOR_CLEAR, ##__VA_ARGS__)

#define log_enter() log_debug("Enter")

#endif // INCLUDE_LOGGER_HPP
