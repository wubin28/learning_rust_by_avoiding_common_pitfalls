install(
    TARGETS dangling_danger_cpp_exe
    RUNTIME COMPONENT dangling_danger_cpp_Runtime
)

if(PROJECT_IS_TOP_LEVEL)
  include(CPack)
endif()
