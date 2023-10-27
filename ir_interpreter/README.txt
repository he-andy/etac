Code for an IR implementation, including an IR simulator.

For Cornell CS 4120, spring 2023.

A test program that builds an IR tree, and then executes it, is
found in src/edu/cornell/cs/cs4120/xic/ir/interpret/Main.java.

  - To build run "./pa4_student_build".
  - You can then run the examples by running:
      java -cp ./build/:./lib/java_cup.jar:/lib/jflex.jar edu.cornell.cs.cs4120.xic.ir.interpret.Main
  - To invoke the interpreter on IR code directly with command-line arguments use:
      java -cp ./build/:./lib/java_cup.jar:/lib/jflex.jar edu.cornell.cs.cs4120.xic.ir.interpret.Cli code.ir arg2 arg3... 

Updates:
  - Build instructions and dependencies added.
  - Build script now builds to separate directory.
  - Specs for visitor code updated.
