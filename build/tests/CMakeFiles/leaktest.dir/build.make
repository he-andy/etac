# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.25

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/local/Cellar/cmake/3.25.2/bin/cmake

# The command to remove a file.
RM = /usr/local/Cellar/cmake/3.25.2/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /Users/evanwilliams/Junior/SP23/4120/etac-emw236/runtime/gc-7.6.4

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build

# Include any dependencies generated for this target.
include tests/CMakeFiles/leaktest.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include tests/CMakeFiles/leaktest.dir/compiler_depend.make

# Include the progress variables for this target.
include tests/CMakeFiles/leaktest.dir/progress.make

# Include the compile flags for this target's objects.
include tests/CMakeFiles/leaktest.dir/flags.make

tests/CMakeFiles/leaktest.dir/leak_test.c.o: tests/CMakeFiles/leaktest.dir/flags.make
tests/CMakeFiles/leaktest.dir/leak_test.c.o: /Users/evanwilliams/Junior/SP23/4120/etac-emw236/runtime/gc-7.6.4/tests/leak_test.c
tests/CMakeFiles/leaktest.dir/leak_test.c.o: tests/CMakeFiles/leaktest.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building C object tests/CMakeFiles/leaktest.dir/leak_test.c.o"
	cd /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/tests && /Library/Developer/CommandLineTools/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT tests/CMakeFiles/leaktest.dir/leak_test.c.o -MF CMakeFiles/leaktest.dir/leak_test.c.o.d -o CMakeFiles/leaktest.dir/leak_test.c.o -c /Users/evanwilliams/Junior/SP23/4120/etac-emw236/runtime/gc-7.6.4/tests/leak_test.c

tests/CMakeFiles/leaktest.dir/leak_test.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/leaktest.dir/leak_test.c.i"
	cd /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/tests && /Library/Developer/CommandLineTools/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /Users/evanwilliams/Junior/SP23/4120/etac-emw236/runtime/gc-7.6.4/tests/leak_test.c > CMakeFiles/leaktest.dir/leak_test.c.i

tests/CMakeFiles/leaktest.dir/leak_test.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/leaktest.dir/leak_test.c.s"
	cd /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/tests && /Library/Developer/CommandLineTools/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /Users/evanwilliams/Junior/SP23/4120/etac-emw236/runtime/gc-7.6.4/tests/leak_test.c -o CMakeFiles/leaktest.dir/leak_test.c.s

# Object files for target leaktest
leaktest_OBJECTS = \
"CMakeFiles/leaktest.dir/leak_test.c.o"

# External object files for target leaktest
leaktest_EXTERNAL_OBJECTS =

tests/leaktest: tests/CMakeFiles/leaktest.dir/leak_test.c.o
tests/leaktest: tests/CMakeFiles/leaktest.dir/build.make
tests/leaktest: libgc-lib.a
tests/leaktest: tests/CMakeFiles/leaktest.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking CXX executable leaktest"
	cd /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/tests && $(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/leaktest.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
tests/CMakeFiles/leaktest.dir/build: tests/leaktest
.PHONY : tests/CMakeFiles/leaktest.dir/build

tests/CMakeFiles/leaktest.dir/clean:
	cd /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/tests && $(CMAKE_COMMAND) -P CMakeFiles/leaktest.dir/cmake_clean.cmake
.PHONY : tests/CMakeFiles/leaktest.dir/clean

tests/CMakeFiles/leaktest.dir/depend:
	cd /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /Users/evanwilliams/Junior/SP23/4120/etac-emw236/runtime/gc-7.6.4 /Users/evanwilliams/Junior/SP23/4120/etac-emw236/runtime/gc-7.6.4/tests /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/tests /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/tests/CMakeFiles/leaktest.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : tests/CMakeFiles/leaktest.dir/depend
