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
include tests/CMakeFiles/realloc_test.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include tests/CMakeFiles/realloc_test.dir/compiler_depend.make

# Include the progress variables for this target.
include tests/CMakeFiles/realloc_test.dir/progress.make

# Include the compile flags for this target's objects.
include tests/CMakeFiles/realloc_test.dir/flags.make

tests/CMakeFiles/realloc_test.dir/realloc_test.c.o: tests/CMakeFiles/realloc_test.dir/flags.make
tests/CMakeFiles/realloc_test.dir/realloc_test.c.o: /Users/evanwilliams/Junior/SP23/4120/etac-emw236/runtime/gc-7.6.4/tests/realloc_test.c
tests/CMakeFiles/realloc_test.dir/realloc_test.c.o: tests/CMakeFiles/realloc_test.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building C object tests/CMakeFiles/realloc_test.dir/realloc_test.c.o"
	cd /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/tests && /Library/Developer/CommandLineTools/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT tests/CMakeFiles/realloc_test.dir/realloc_test.c.o -MF CMakeFiles/realloc_test.dir/realloc_test.c.o.d -o CMakeFiles/realloc_test.dir/realloc_test.c.o -c /Users/evanwilliams/Junior/SP23/4120/etac-emw236/runtime/gc-7.6.4/tests/realloc_test.c

tests/CMakeFiles/realloc_test.dir/realloc_test.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/realloc_test.dir/realloc_test.c.i"
	cd /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/tests && /Library/Developer/CommandLineTools/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /Users/evanwilliams/Junior/SP23/4120/etac-emw236/runtime/gc-7.6.4/tests/realloc_test.c > CMakeFiles/realloc_test.dir/realloc_test.c.i

tests/CMakeFiles/realloc_test.dir/realloc_test.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/realloc_test.dir/realloc_test.c.s"
	cd /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/tests && /Library/Developer/CommandLineTools/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /Users/evanwilliams/Junior/SP23/4120/etac-emw236/runtime/gc-7.6.4/tests/realloc_test.c -o CMakeFiles/realloc_test.dir/realloc_test.c.s

# Object files for target realloc_test
realloc_test_OBJECTS = \
"CMakeFiles/realloc_test.dir/realloc_test.c.o"

# External object files for target realloc_test
realloc_test_EXTERNAL_OBJECTS =

tests/realloc_test: tests/CMakeFiles/realloc_test.dir/realloc_test.c.o
tests/realloc_test: tests/CMakeFiles/realloc_test.dir/build.make
tests/realloc_test: libgc-lib.a
tests/realloc_test: tests/CMakeFiles/realloc_test.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking CXX executable realloc_test"
	cd /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/tests && $(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/realloc_test.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
tests/CMakeFiles/realloc_test.dir/build: tests/realloc_test
.PHONY : tests/CMakeFiles/realloc_test.dir/build

tests/CMakeFiles/realloc_test.dir/clean:
	cd /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/tests && $(CMAKE_COMMAND) -P CMakeFiles/realloc_test.dir/cmake_clean.cmake
.PHONY : tests/CMakeFiles/realloc_test.dir/clean

tests/CMakeFiles/realloc_test.dir/depend:
	cd /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /Users/evanwilliams/Junior/SP23/4120/etac-emw236/runtime/gc-7.6.4 /Users/evanwilliams/Junior/SP23/4120/etac-emw236/runtime/gc-7.6.4/tests /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/tests /Users/evanwilliams/Junior/SP23/4120/etac-emw236/build/tests/CMakeFiles/realloc_test.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : tests/CMakeFiles/realloc_test.dir/depend

