

package edu.cornell.cs.cs4120.xic.ir.parse;

import edu.cornell.cs.cs4120.xic.ir.*;
import java_cup.runtime.ComplexSymbolFactory;
import java.util.ArrayList;
import java.util.Collections;
import java.util.LinkedList;
import java.util.List;

/** CUP v0.11b 20150326 generated parser.
  */
public class IRParser
 extends java_cup.runtime.lr_parser {

  @Override
  public final Class<?> getSymbolContainer() {
    return IRSym.class;
  }

  /** Default constructor. */
  @Deprecated
  public IRParser() {super();}

  /** Constructor which sets the default scanner. */
  @Deprecated
  public IRParser(java_cup.runtime.Scanner s) {super(s);}

  /** Constructor which sets the default scanner and a SymbolFactory. */
  public IRParser(java_cup.runtime.Scanner s, java_cup.runtime.SymbolFactory sf) {super(s,sf);}

  /** Production table. */
  protected static final short _production_table[][] = 
    unpackFromStrings(new String[] {
    "\000\102\000\002\002\004\000\002\002\011\000\002\012" +
    "\002\000\002\012\003\000\002\013\003\000\002\013\004" +
    "\000\002\015\002\000\002\015\003\000\002\016\003\000" +
    "\002\016\004\000\002\017\007\000\002\020\002\000\002" +
    "\020\003\000\002\021\003\000\002\021\004\000\002\003" +
    "\003\000\002\003\004\000\002\004\007\000\002\014\003" +
    "\000\002\006\005\000\002\007\005\000\002\007\005\000" +
    "\002\007\004\000\002\007\004\000\002\007\004\000\002" +
    "\007\006\000\002\007\005\000\002\007\004\000\002\007" +
    "\005\000\002\007\004\000\002\025\006\000\002\025\006" +
    "\000\002\022\005\000\002\023\004\000\002\023\004\000" +
    "\002\023\003\000\002\023\004\000\002\023\005\000\002" +
    "\023\004\000\002\023\005\000\002\026\003\000\002\024" +
    "\005\000\002\024\005\000\002\024\005\000\002\024\005" +
    "\000\002\024\005\000\002\024\005\000\002\024\005\000" +
    "\002\024\005\000\002\024\005\000\002\024\005\000\002" +
    "\024\005\000\002\024\005\000\002\024\005\000\002\024" +
    "\005\000\002\024\005\000\002\024\005\000\002\024\005" +
    "\000\002\024\005\000\002\024\005\000\002\005\003\000" +
    "\002\005\004\000\002\010\002\000\002\010\003\000\002" +
    "\011\003\000\002\011\004" });

  /** Access to production table. */
  @Override
  public short[][] production_table() {return _production_table;}

  /** Parse-action table. */
  protected static final short[][] _action_table = 
    unpackFromStrings(new String[] {
    "\000\221\000\004\004\004\001\002\000\004\051\007\001" +
    "\002\000\004\002\006\001\002\000\004\002\001\001\002" +
    "\000\004\052\011\001\002\000\010\004\uffff\050\uffff\052" +
    "\011\001\002\000\012\004\uffef\005\uffef\050\uffef\052\uffef" +
    "\001\002\000\006\004\ufffb\050\020\001\002\000\010\004" +
    "\ufffd\050\ufffd\052\ufffd\001\002\000\010\004\ufffe\050\ufffe" +
    "\052\011\001\002\000\010\004\ufffc\050\ufffc\052\ufffc\001" +
    "\002\000\004\004\034\001\002\000\006\004\ufff9\050\ufff9" +
    "\001\002\000\004\052\023\001\002\000\006\004\ufffa\050" +
    "\020\001\002\000\006\004\ufff8\050\ufff8\001\002\000\004" +
    "\004\024\001\002\000\006\005\ufff6\053\025\001\002\000" +
    "\006\005\uffd9\053\uffd9\001\002\000\004\005\032\001\002" +
    "\000\006\005\ufff4\053\ufff4\001\002\000\006\005\ufff5\053" +
    "\025\001\002\000\006\005\ufff3\053\ufff3\001\002\000\006" +
    "\004\ufff7\050\ufff7\001\002\000\006\004\ufff2\005\ufff2\001" +
    "\002\000\004\047\040\001\002\000\006\004\034\005\037" +
    "\001\002\000\006\004\ufff1\005\ufff1\001\002\000\004\002" +
    "\000\001\002\000\004\052\011\001\002\000\004\004\042" +
    "\001\002\000\022\006\050\007\051\010\053\011\055\012" +
    "\054\013\047\014\052\015\046\001\002\000\004\005\044" +
    "\001\002\000\006\004\ufff0\005\ufff0\001\002\000\004\005" +
    "\223\001\002\000\006\004\221\005\uffc3\001\002\000\004" +
    "\004\061\001\002\000\004\004\204\001\002\000\004\004" +
    "\061\001\002\000\004\052\011\001\002\000\004\004\061" +
    "\001\002\000\004\004\061\001\002\000\004\004\042\001" +
    "\002\000\006\004\042\005\uffea\001\002\000\006\004\uffc5" +
    "\005\uffc5\001\002\000\006\004\uffc4\005\uffc4\001\002\000" +
    "\064\016\067\017\105\020\106\021\064\022\101\023\113" +
    "\024\063\025\072\026\075\027\115\030\103\031\073\032" +
    "\110\033\074\034\112\035\107\036\102\037\065\040\077" +
    "\041\114\042\066\043\070\044\100\045\111\046\104\001" +
    "\002\000\004\005\uffe9\001\002\000\004\004\061\001\002" +
    "\000\004\004\061\001\002\000\004\004\061\001\002\000" +
    "\004\004\061\001\002\000\004\053\025\001\002\000\004" +
    "\004\061\001\002\000\004\005\uffde\001\002\000\004\004" +
    "\061\001\002\000\004\004\061\001\002\000\004\004\061" +
    "\001\002\000\004\004\061\001\002\000\004\005\151\001" +
    "\002\000\004\004\061\001\002\000\004\004\061\001\002" +
    "\000\004\052\011\001\002\000\004\004\061\001\002\000" +
    "\004\004\061\001\002\000\004\004\061\001\002\000\004" +
    "\052\011\001\002\000\004\004\061\001\002\000\004\004" +
    "\061\001\002\000\004\004\061\001\002\000\004\004\061" +
    "\001\002\000\004\004\061\001\002\000\004\004\042\001" +
    "\002\000\004\004\061\001\002\000\004\004\061\001\002" +
    "\000\004\004\061\001\002\000\004\005\uffd5\001\002\000" +
    "\004\004\061\001\002\000\004\005\uffcb\001\002\000\004" +
    "\004\061\001\002\000\004\005\uffda\001\002\000\004\004" +
    "\061\001\002\000\004\005\uffd0\001\002\000\004\004\061" +
    "\001\002\000\004\005\uffc7\001\002\000\004\004\061\001" +
    "\002\000\004\005\uffd2\001\002\000\004\004\061\001\002" +
    "\000\004\005\uffcf\001\002\000\004\005\uffdd\001\002\000" +
    "\004\005\uffdf\001\002\000\004\004\061\001\002\000\004" +
    "\005\uffc6\001\002\000\004\004\061\001\002\000\004\005" +
    "\uffd4\001\002\000\004\004\061\001\002\000\004\005\uffce" +
    "\001\002\000\004\005\uffdb\001\002\000\004\004\061\001" +
    "\002\000\004\005\uffc8\001\002\000\004\004\061\001\002" +
    "\000\004\005\uffcc\001\002\000\010\004\uffe1\005\uffe1\052" +
    "\uffe1\001\002\000\004\004\061\001\002\000\004\005\uffd6" +
    "\001\002\000\004\004\061\001\002\000\004\005\uffd1\001" +
    "\002\000\004\004\061\001\002\000\004\005\uffd3\001\002" +
    "\000\004\004\061\001\002\000\004\005\uffd7\001\002\000" +
    "\004\004\061\001\002\000\004\005\uffc9\001\002\000\004" +
    "\005\uffe0\001\002\000\004\004\061\001\002\000\004\005" +
    "\uffca\001\002\000\004\004\061\001\002\000\004\005\uffcd" +
    "\001\002\000\006\004\061\005\uffc3\001\002\000\004\005" +
    "\uffdc\001\002\000\006\004\061\005\uffc2\001\002\000\006" +
    "\004\uffc1\005\uffc1\001\002\000\006\004\uffc0\005\uffc0\001" +
    "\002\000\004\004\061\001\002\000\004\005\uffd8\001\002" +
    "\000\004\005\uffeb\001\002\000\004\005\uffe6\001\002\000" +
    "\006\004\061\005\uffc3\001\002\000\004\005\uffec\001\002" +
    "\000\006\017\207\020\210\001\002\000\004\004\061\001" +
    "\002\000\004\005\uffed\001\002\000\004\052\011\001\002" +
    "\000\004\004\061\001\002\000\004\005\212\001\002\000" +
    "\004\004\uffe2\001\002\000\004\005\214\001\002\000\004" +
    "\004\uffe3\001\002\000\004\052\011\001\002\000\006\005" +
    "\uffe7\052\011\001\002\000\004\005\uffe8\001\002\000\004" +
    "\005\uffe4\001\002\000\066\005\222\016\067\017\105\020" +
    "\106\021\064\022\101\023\113\024\063\025\072\026\075" +
    "\027\115\030\103\031\073\032\110\033\074\034\112\035" +
    "\107\036\102\037\065\040\077\041\114\042\066\043\070" +
    "\044\100\045\111\046\104\001\002\000\004\005\uffe5\001" +
    "\002\000\006\004\uffee\005\uffee\001\002" });

  /** Access to parse-action table. */
  @Override
  public short[][] action_table() {return _action_table;}

  /** {@code reduce_goto} table. */
  protected static final short[][] _reduce_table = 
    unpackFromStrings(new String[] {
    "\000\221\000\004\002\004\001\001\000\002\001\001\000" +
    "\002\001\001\000\002\001\001\000\004\014\007\001\001" +
    "\000\010\012\011\013\013\014\012\001\001\000\002\001" +
    "\001\000\010\015\015\016\020\017\016\001\001\000\002" +
    "\001\001\000\004\014\014\001\001\000\002\001\001\000" +
    "\006\003\034\004\032\001\001\000\002\001\001\000\002" +
    "\001\001\000\004\017\021\001\001\000\002\001\001\000" +
    "\002\001\001\000\010\020\025\021\027\026\026\001\001" +
    "\000\002\001\001\000\002\001\001\000\002\001\001\000" +
    "\004\026\030\001\001\000\002\001\001\000\002\001\001" +
    "\000\002\001\001\000\002\001\001\000\004\004\035\001" +
    "\001\000\002\001\001\000\002\001\001\000\004\014\040" +
    "\001\001\000\004\006\042\001\001\000\004\007\044\001" +
    "\001\000\002\001\001\000\002\001\001\000\002\001\001" +
    "\000\010\010\217\011\172\022\173\001\001\000\004\022" +
    "\214\001\001\000\004\025\204\001\001\000\004\022\201" +
    "\001\001\000\004\014\200\001\001\000\004\022\177\001" +
    "\001\000\004\022\061\001\001\000\006\005\055\006\056" +
    "\001\001\000\004\006\057\001\001\000\002\001\001\000" +
    "\002\001\001\000\006\023\075\024\070\001\001\000\002" +
    "\001\001\000\004\022\175\001\001\000\004\022\170\001" +
    "\001\000\004\022\166\001\001\000\004\022\164\001\001" +
    "\000\004\026\163\001\001\000\004\022\161\001\001\000" +
    "\002\001\001\000\004\022\157\001\001\000\004\022\155" +
    "\001\001\000\004\022\153\001\001\000\004\022\151\001" +
    "\001\000\002\001\001\000\004\022\146\001\001\000\004" +
    "\022\144\001\001\000\004\014\143\001\001\000\004\022" +
    "\141\001\001\000\004\022\137\001\001\000\004\022\135" +
    "\001\001\000\004\014\134\001\001\000\004\022\133\001" +
    "\001\000\004\022\131\001\001\000\004\022\127\001\001" +
    "\000\004\022\125\001\001\000\004\022\123\001\001\000" +
    "\004\006\121\001\001\000\004\022\117\001\001\000\004" +
    "\022\115\001\001\000\004\022\116\001\001\000\002\001" +
    "\001\000\004\022\120\001\001\000\002\001\001\000\004" +
    "\022\122\001\001\000\002\001\001\000\004\022\124\001" +
    "\001\000\002\001\001\000\004\022\126\001\001\000\002" +
    "\001\001\000\004\022\130\001\001\000\002\001\001\000" +
    "\004\022\132\001\001\000\002\001\001\000\002\001\001" +
    "\000\002\001\001\000\004\022\136\001\001\000\002\001" +
    "\001\000\004\022\140\001\001\000\002\001\001\000\004" +
    "\022\142\001\001\000\002\001\001\000\002\001\001\000" +
    "\004\022\145\001\001\000\002\001\001\000\004\022\147" +
    "\001\001\000\002\001\001\000\002\001\001\000\004\022" +
    "\152\001\001\000\002\001\001\000\004\022\154\001\001" +
    "\000\002\001\001\000\004\022\156\001\001\000\002\001" +
    "\001\000\004\022\160\001\001\000\002\001\001\000\004" +
    "\022\162\001\001\000\002\001\001\000\002\001\001\000" +
    "\004\022\165\001\001\000\002\001\001\000\004\022\167" +
    "\001\001\000\002\001\001\000\010\010\171\011\172\022" +
    "\173\001\001\000\002\001\001\000\004\022\174\001\001" +
    "\000\002\001\001\000\002\001\001\000\004\022\176\001" +
    "\001\000\002\001\001\000\002\001\001\000\002\001\001" +
    "\000\010\010\202\011\172\022\173\001\001\000\002\001" +
    "\001\000\002\001\001\000\004\022\205\001\001\000\002" +
    "\001\001\000\004\014\212\001\001\000\004\022\210\001" +
    "\001\000\002\001\001\000\002\001\001\000\002\001\001" +
    "\000\002\001\001\000\004\014\215\001\001\000\004\014" +
    "\216\001\001\000\002\001\001\000\002\001\001\000\006" +
    "\023\075\024\070\001\001\000\002\001\001\000\002\001" +
    "\001" });

  /** Access to {@code reduce_goto} table. */
  @Override
  public short[][] reduce_table() {return _reduce_table;}

  /** Instance of action encapsulation class. */
  protected CUP$IRParser$actions action_obj;

  /** Action encapsulation object initializer. */
  @Override
  protected void init_actions()
    {
      action_obj = new CUP$IRParser$actions(this);
    }

  /** Invoke a user supplied parse action. */
  @Override
  public java_cup.runtime.Symbol do_action(
    int                        act_num,
    java_cup.runtime.lr_parser parser,
    java.util.Stack<java_cup.runtime.Symbol> stack,
    int                        top)
    throws java.lang.Exception
  {
    /* call code in generated class */
    return action_obj.CUP$IRParser$do_action(act_num, parser, stack, top);
  }

  /** Indicates start state. */
  @Override
  public int start_state() {return 0;}
  /** Indicates start production. */
  @Override
  public int start_production() {return 0;}

  /** {@code EOF} Symbol index. */
  @Override
  public int EOF_sym() {return 0;}

  /** {@code error} Symbol index. */
  @Override
  public int error_sym() {return 1;}




    protected IRNodeFactory nf;

    public IRParser(IRLexer lexer, IRNodeFactory nf) {
        super(lexer, new ComplexSymbolFactory());
        this.nf = nf;
    }


/** Cup generated class to encapsulate user supplied action code.*/
class CUP$IRParser$actions {
    private final IRParser parser;

    /** Constructor */
    CUP$IRParser$actions(IRParser parser) {
        this.parser = parser;
    }

    /** Method with the actual generated action code for actions 0 to 65. */
    public final java_cup.runtime.Symbol CUP$IRParser$do_action_part00000000(
            int                        CUP$IRParser$act_num,
            java_cup.runtime.lr_parser CUP$IRParser$parser,
            java.util.Stack<java_cup.runtime.Symbol> CUP$IRParser$stack,
            int                        CUP$IRParser$top)
            throws java.lang.Exception {
            /* Symbol object for return from actions */
            java_cup.runtime.Symbol CUP$IRParser$result;

        /* select the action based on the action number */
        switch (CUP$IRParser$act_num) {
        /*. . . . . . . . . . . . . . . . . . . .*/
        case 0: // $START ::= compunit EOF 
            {
                Object RESULT = null;
                IRCompUnit start_val = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRCompUnit> value();
                RESULT = start_val;
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("$START",0, RESULT);
            }
            /* ACCEPT */
            CUP$IRParser$parser.done_parsing();
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 1: // compunit ::= LPAREN COMPUNIT name ctor_list data_list funcdecls RPAREN 
            {
                IRCompUnit RESULT = null;
                String n = CUP$IRParser$stack.elementAt(CUP$IRParser$top-4).<String> value();
                List<String> c_list = CUP$IRParser$stack.elementAt(CUP$IRParser$top-3).<List<String>> value();
                List<IRData> d_list = CUP$IRParser$stack.elementAt(CUP$IRParser$top-2).<List<IRData>> value();
                List<IRFuncDecl> f = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<List<IRFuncDecl>> value();
                
        RESULT = parser.nf.IRCompUnit(n);
        for (String ctor: c_list)
             RESULT.appendCtor(ctor);
        for (IRData data : d_list)
             RESULT.appendData(data);
        for (IRFuncDecl fd : f)
             RESULT.appendFunc(fd);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("compunit",0, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 2: // ctor_list ::= 
            {
                List<String> RESULT = null;
                 RESULT = Collections.emptyList(); 
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("ctor_list",8, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 3: // ctor_list ::= ctor_list_non_empty 
            {
                List<String> RESULT = null;
                List<String> l = CUP$IRParser$stack.peek().<List<String>> value();
                 RESULT = l; 
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("ctor_list",8, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 4: // ctor_list_non_empty ::= name 
            {
                List<String> RESULT = null;
                String c = CUP$IRParser$stack.peek().<String> value();
                
        RESULT = new ArrayList<>();
        RESULT.add(c);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("ctor_list_non_empty",9, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 5: // ctor_list_non_empty ::= ctor_list_non_empty name 
            {
                List<String> RESULT = null;
                List<String> c_list = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<List<String>> value();
                String c = CUP$IRParser$stack.peek().<String> value();
                
        RESULT = c_list;
        RESULT.add(c);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("ctor_list_non_empty",9, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 6: // data_list ::= 
            {
                List<IRData> RESULT = null;
                 RESULT = Collections.emptyList(); 
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("data_list",11, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 7: // data_list ::= data_list_non_empty 
            {
                List<IRData> RESULT = null;
                List<IRData> l = CUP$IRParser$stack.peek().<List<IRData>> value();
                 RESULT = l; 
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("data_list",11, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 8: // data_list_non_empty ::= data 
            {
                List<IRData> RESULT = null;
                IRData d = CUP$IRParser$stack.peek().<IRData> value();
                
        RESULT = new ArrayList<>();
        RESULT.add(d);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("data_list_non_empty",12, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 9: // data_list_non_empty ::= data_list_non_empty data 
            {
                List<IRData> RESULT = null;
                List<IRData> d_list = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<List<IRData>> value();
                IRData d = CUP$IRParser$stack.peek().<IRData> value();
                
        RESULT = d_list;
        RESULT.add(d);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("data_list_non_empty",12, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 10: // data ::= DATA ATOM LPAREN number_list RPAREN 
            {
                IRData RESULT = null;
                String a = CUP$IRParser$stack.elementAt(CUP$IRParser$top-3).<String> value();
                List<Long> n_list = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<List<Long>> value();
                
    int s = n_list.size();
    long[] rawData = new long[s];
    for (int i = 0; i < s; i++) {
        rawData[i] = n_list.get(i);
    }
    RESULT = new IRData(a, rawData);

                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("data",13, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 11: // number_list ::= 
            {
                List<Long> RESULT = null;
                 RESULT = Collections.emptyList(); 
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("number_list",14, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 12: // number_list ::= number_list_non_empty 
            {
                List<Long> RESULT = null;
                List<Long> l = CUP$IRParser$stack.peek().<List<Long>> value();
                 RESULT = l; 
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("number_list",14, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 13: // number_list_non_empty ::= num 
            {
                List<Long> RESULT = null;
                Long n = CUP$IRParser$stack.peek().<Long> value();
                
        RESULT = new ArrayList<>();
        RESULT.add(n);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("number_list_non_empty",15, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 14: // number_list_non_empty ::= number_list_non_empty num 
            {
                List<Long> RESULT = null;
                List<Long> n_list = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<List<Long>> value();
                Long n = CUP$IRParser$stack.peek().<Long> value();
                
        RESULT = n_list;
        RESULT.add(n);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("number_list_non_empty",15, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 15: // funcdecls ::= funcdecl 
            {
                List<IRFuncDecl> RESULT = null;
                IRFuncDecl fd = CUP$IRParser$stack.peek().<IRFuncDecl> value();
                
        RESULT = new ArrayList<>();
        RESULT.add(fd);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("funcdecls",1, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 16: // funcdecls ::= funcdecls funcdecl 
            {
                List<IRFuncDecl> RESULT = null;
                List<IRFuncDecl> f = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<List<IRFuncDecl>> value();
                IRFuncDecl fd = CUP$IRParser$stack.peek().<IRFuncDecl> value();
                
        RESULT = f;
        RESULT.add(fd);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("funcdecls",1, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 17: // funcdecl ::= LPAREN FUNC name stmt RPAREN 
            {
                IRFuncDecl RESULT = null;
                String n = CUP$IRParser$stack.elementAt(CUP$IRParser$top-2).<String> value();
                IRStmt s = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRStmt> value();
                
        RESULT = parser.nf.IRFuncDecl(n, s);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("funcdecl",2, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 18: // name ::= ATOM 
            {
                String RESULT = null;
                String a = CUP$IRParser$stack.peek().<String> value();
                
        RESULT = a;
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("name",10, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 19: // stmt ::= LPAREN bare_stmt RPAREN 
            {
                IRStmt RESULT = null;
                IRStmt s = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRStmt> value();
                
        RESULT = s;
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("stmt",4, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 20: // bare_stmt ::= MOVE dest expr 
            {
                IRStmt RESULT = null;
                IRExpr dest = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr e = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRMove(dest, e);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_stmt",5, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 21: // bare_stmt ::= CALL_STMT expr exprs_opt 
            {
                IRStmt RESULT = null;
                IRExpr target = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                List<IRExpr> args = CUP$IRParser$stack.peek().<List<IRExpr>> value();
                
        RESULT = parser.nf.IRCallStmt(target, args);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_stmt",5, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 22: // bare_stmt ::= EXP expr 
            {
                IRStmt RESULT = null;
                IRExpr e = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRExp(e);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_stmt",5, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 23: // bare_stmt ::= SEQ stmts 
            {
                IRStmt RESULT = null;
                List<IRStmt> l = CUP$IRParser$stack.peek().<List<IRStmt>> value();
                
        RESULT = parser.nf.IRSeq(l);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_stmt",5, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 24: // bare_stmt ::= JUMP expr 
            {
                IRStmt RESULT = null;
                IRExpr e = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRJump(e);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_stmt",5, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 25: // bare_stmt ::= CJUMP expr name name 
            {
                IRStmt RESULT = null;
                IRExpr e = CUP$IRParser$stack.elementAt(CUP$IRParser$top-2).<IRExpr> value();
                String trueLabel = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<String> value();
                String falseLabel = CUP$IRParser$stack.peek().<String> value();
                
        RESULT = parser.nf.IRCJump(e, trueLabel, falseLabel);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_stmt",5, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 26: // bare_stmt ::= CJUMP expr name 
            {
                IRStmt RESULT = null;
                IRExpr e = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                String trueLabel = CUP$IRParser$stack.peek().<String> value();
                
        RESULT = parser.nf.IRCJump(e, trueLabel);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_stmt",5, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 27: // bare_stmt ::= LABEL name 
            {
                IRStmt RESULT = null;
                String n = CUP$IRParser$stack.peek().<String> value();
                
        RESULT = parser.nf.IRLabel(n);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_stmt",5, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 28: // bare_stmt ::= RETURN LPAREN RPAREN 
            {
                IRStmt RESULT = null;
                
        RESULT = parser.nf.IRReturn(Collections.emptyList());
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_stmt",5, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 29: // bare_stmt ::= RETURN exprs_opt 
            {
                IRStmt RESULT = null;
                List<IRExpr> args = CUP$IRParser$stack.peek().<List<IRExpr>> value();
                
        RESULT = parser.nf.IRReturn(args);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_stmt",5, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 30: // dest ::= LPAREN TEMP name RPAREN 
            {
                IRExpr RESULT = null;
                String n = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<String> value();
                
        RESULT = parser.nf.IRTemp(n);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("dest",19, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 31: // dest ::= LPAREN MEM expr RPAREN 
            {
                IRExpr RESULT = null;
                IRExpr e = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                
        RESULT = parser.nf.IRMem(e);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("dest",19, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 32: // expr ::= LPAREN bare_expr RPAREN 
            {
                IRExpr RESULT = null;
                IRExpr e = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                
        RESULT = e;
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("expr",16, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 33: // bare_expr ::= CONST num 
            {
                IRExpr RESULT = null;
                Long n = CUP$IRParser$stack.peek().<Long> value();
                
        RESULT = parser.nf.IRConst(n);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_expr",17, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 34: // bare_expr ::= TEMP name 
            {
                IRExpr RESULT = null;
                String n = CUP$IRParser$stack.peek().<String> value();
                
        RESULT = parser.nf.IRTemp(n);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_expr",17, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 35: // bare_expr ::= op 
            {
                IRExpr RESULT = null;
                IRExpr o = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = o;
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_expr",17, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 36: // bare_expr ::= MEM expr 
            {
                IRExpr RESULT = null;
                IRExpr e = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRMem(e);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_expr",17, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 37: // bare_expr ::= CALL expr exprs_opt 
            {
                IRExpr RESULT = null;
                IRExpr target = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                List<IRExpr> args = CUP$IRParser$stack.peek().<List<IRExpr>> value();
                
        RESULT = parser.nf.IRCall(target, args);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_expr",17, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 38: // bare_expr ::= NAME name 
            {
                IRExpr RESULT = null;
                String n = CUP$IRParser$stack.peek().<String> value();
                
        RESULT = parser.nf.IRName(n);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_expr",17, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 39: // bare_expr ::= ESEQ stmt expr 
            {
                IRExpr RESULT = null;
                IRStmt s = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRStmt> value();
                IRExpr e = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRESeq(s, e);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("bare_expr",17, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 40: // num ::= NUMBER 
            {
                Long RESULT = null;
                Long n = CUP$IRParser$stack.peek().<Long> value();
                
        RESULT = n;
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("num",20, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 41: // op ::= ADD expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.ADD, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 42: // op ::= SUB expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.SUB, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 43: // op ::= MUL expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.MUL, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 44: // op ::= HMUL expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.HMUL, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 45: // op ::= DIV expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.DIV, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 46: // op ::= MOD expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.MOD, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 47: // op ::= AND expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.AND, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 48: // op ::= OR expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.OR, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 49: // op ::= XOR expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.XOR, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 50: // op ::= LSHIFT expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.LSHIFT, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 51: // op ::= RSHIFT expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.RSHIFT, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 52: // op ::= ARSHIFT expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.ARSHIFT, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 53: // op ::= EQ expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.EQ, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 54: // op ::= NEQ expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.NEQ, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 55: // op ::= LT expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.LT, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 56: // op ::= ULT expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.ULT, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 57: // op ::= GT expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.GT, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 58: // op ::= LEQ expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.LEQ, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 59: // op ::= GEQ expr expr 
            {
                IRExpr RESULT = null;
                IRExpr x = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<IRExpr> value();
                IRExpr y = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = parser.nf.IRBinOp(IRBinOp.OpType.GEQ, x,y);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("op",18, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 60: // stmts ::= stmt 
            {
                List<IRStmt> RESULT = null;
                IRStmt s = CUP$IRParser$stack.peek().<IRStmt> value();
                
        RESULT = new LinkedList<>();
        RESULT.add(s);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("stmts",3, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 61: // stmts ::= stmts stmt 
            {
                List<IRStmt> RESULT = null;
                List<IRStmt> l = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<List<IRStmt>> value();
                IRStmt s = CUP$IRParser$stack.peek().<IRStmt> value();
                
        RESULT = l;
        RESULT.add(s);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("stmts",3, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 62: // exprs_opt ::= 
            {
                List<IRExpr> RESULT = null;
                
        RESULT = Collections.emptyList();
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("exprs_opt",6, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 63: // exprs_opt ::= exprs 
            {
                List<IRExpr> RESULT = null;
                List<IRExpr> l = CUP$IRParser$stack.peek().<List<IRExpr>> value();
                
        RESULT = l;
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("exprs_opt",6, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 64: // exprs ::= expr 
            {
                List<IRExpr> RESULT = null;
                IRExpr e = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = new LinkedList<>();
        RESULT.add(e);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("exprs",7, RESULT);
            }
            return CUP$IRParser$result;

        /*. . . . . . . . . . . . . . . . . . . .*/
        case 65: // exprs ::= exprs expr 
            {
                List<IRExpr> RESULT = null;
                List<IRExpr> l = CUP$IRParser$stack.elementAt(CUP$IRParser$top-1).<List<IRExpr>> value();
                IRExpr e = CUP$IRParser$stack.peek().<IRExpr> value();
                
        RESULT = l;
        RESULT.add(e);
    
                CUP$IRParser$result = parser.getSymbolFactory().newSymbol("exprs",7, RESULT);
            }
            return CUP$IRParser$result;

        /* . . . . . .*/
        default:
            throw new Exception(
                  "Invalid action number " + CUP$IRParser$act_num + " found in internal parse table");

        }
    } /* end of method */

    /** Method splitting the generated action code into several parts. */
    public final java_cup.runtime.Symbol CUP$IRParser$do_action(
            int                        CUP$IRParser$act_num,
            java_cup.runtime.lr_parser CUP$IRParser$parser,
            java.util.Stack<java_cup.runtime.Symbol> CUP$IRParser$stack,
            int                        CUP$IRParser$top)
            throws java.lang.Exception {
            return CUP$IRParser$do_action_part00000000(
                           CUP$IRParser$act_num,
                           CUP$IRParser$parser,
                           CUP$IRParser$stack,
                           CUP$IRParser$top);
    }
}

}
