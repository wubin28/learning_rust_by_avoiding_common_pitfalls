# Parent project does not export its library target, so this CML implicitly
# depends on being added from it, i.e. the testing is done only from the build
# tree and is not feasible from an install location

project(dangling_danger_cppTests LANGUAGES CXX)

# ---- Tests ----

add_executable(dangling_danger_cpp_test source/dangling_danger_cpp_test.cpp)
target_link_libraries(dangling_danger_cpp_test PRIVATE dangling_danger_cpp_lib)
target_compile_features(dangling_danger_cpp_test PRIVATE cxx_std_20)

add_test(NAME dangling_danger_cpp_test COMMAND dangling_danger_cpp_test)

# ---- End-of-file commands ----

add_folders(Test)
