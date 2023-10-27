package edu.cornell.cs.cs4120.xic.ir.interpret;

import edu.cornell.cs.cs4120.util.CodeWriterSExpPrinter;
import edu.cornell.cs.cs4120.util.SExpPrinter;
import edu.cornell.cs.cs4120.xic.ir.IRBinOp;
import edu.cornell.cs.cs4120.xic.ir.IRBinOp.OpType;
import edu.cornell.cs.cs4120.xic.ir.IRCallStmt;
import edu.cornell.cs.cs4120.xic.ir.IRCompUnit;
import edu.cornell.cs.cs4120.xic.ir.IRConst;
import edu.cornell.cs.cs4120.xic.ir.IRFuncDecl;
import edu.cornell.cs.cs4120.xic.ir.IRMove;
import edu.cornell.cs.cs4120.xic.ir.IRName;
import edu.cornell.cs.cs4120.xic.ir.IRNodeFactory_c;
import edu.cornell.cs.cs4120.xic.ir.IRReturn;
import edu.cornell.cs.cs4120.xic.ir.IRSeq;
import edu.cornell.cs.cs4120.xic.ir.IRStmt;
import edu.cornell.cs.cs4120.xic.ir.IRTemp;
import edu.cornell.cs.cs4120.xic.ir.parse.IRLexer;
import edu.cornell.cs.cs4120.xic.ir.parse.IRParser;
import edu.cornell.cs.cs4120.xic.ir.visit.CheckCanonicalIRVisitor;
import edu.cornell.cs.cs4120.xic.ir.visit.CheckConstFoldedIRVisitor;

import java.io.PrintWriter;
import java.io.StringReader;
import java.io.StringWriter;

public class Main {

    public static void main(String[] args) {
        // run example from the pa4 spec
        IRCompUnit sexpr = constructIR(args[0]);
        // System.out.println("Code:");
        // String prettyPrintedProgram = prettyPrint(sexpr);
        // System.out.println(prettyPrintedProgram);
        IRSimulator sim = new IRSimulator(sexpr);
        // System.out.println("Output:");
        long result = sim.call("_Imain_paai", 0);
    }

    private static IRCompUnit constructIR(String ir_string) {
        return parse(ir_string);
    }

    private static String prettyPrint(IRCompUnit compUnit) {
        StringWriter sw = new StringWriter();
        try (PrintWriter pw = new PrintWriter(sw);
                SExpPrinter sp = new CodeWriterSExpPrinter(pw)) {
            compUnit.printSExp(sp);
        }
        return sw.toString();
    }

    private static IRCompUnit parse(String prog) {
        try (StringReader r = new StringReader(prog)) {
            IRParser parser = new IRParser(new IRLexer(r), new IRNodeFactory_c());
            try {
                return parser.parse().<IRCompUnit>value();
            } catch (RuntimeException e) {
                throw e;
            } catch (Exception e) {
                // Used by CUP to indicate an unrecoverable error.
                String msg = e.getMessage();
                if (msg != null) System.err.println("Syntax error: " + msg);
                return null;
            }
        }
    }
}
