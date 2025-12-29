use crate::parser::instr_gen::gen_instructions;

gen_instructions! {
    Statement,
    0i0o:
        Noop("nop")  = "No-op"
        Stop("stop") = "Stop execution"
        End("end")   = "End execution"

        UCIdle("ucontrol" "unbind")   = "Unit idle"
        UCStop("ucontrol" "stop")     = "Unit stop"
        UCUnbind("ucontrol" "unbind") = "Unit unbind"

        UCAutoPathfind("ucontrol" "autoPathFind") = "Unit auto pathfind"
        UCPayloadDrop("ucontrol" "payDrop")       = "Unit drop payload"
        UCPayloadEnter("ucontrol" "payEnter")     = "Unit enter payload containing block"

        DrawReset("draw" "reset") = ""
    ---

    1i0o:
        DrawCol("draw" "col")       = "Set draw colour"
        DrawStroke("draw" "stroke") = "Set draw stroke size"

        Print("print")         = "Print a string"
        PrintChar("printchar") = "Print a char based on its ASCII code"
        Format("format")       = "Print a string with format substitutions"

        DrawFlush("drawflush")   = "Draw the drawbuffer to a display"
        PrintFlush("printflush") = "Print the messagebuffer to a message block"

        UBind("ubind") = "Bind a unit with a given type"

        UCBoost("ucontrol" "boost")   = "Set whether a unit should boost"
        UCPayTake("ucontrol" "payTake") = "Make a unit take payload"
        UCFlag("ucontrol" "flag")     = "Sets a unit's flag"

        Wait("wait") = "Wait for n seconds"
    ---

    2i0o:
        DrawTranslate("draw" "translate") = "Translate everything in the print buffer"
        DrawRotate("draw" "rotate")       = "Rotate everything in the print buffer"
        DrawScale("draw" "scale")         = "Scale everything in the print buffer"

        ControlEnabled("control" "enabled") = "Set whether a block is enabled or not"
        ControlConfig("control" "config")   = "Set the configuration of a block (exact behaviour depends on the block)"
        ControlColour("control" "color")    = "Set the colour of a block that supports it"

        UCMove("ucontrol" "move")         = "Set the position for units to move to"
        UCPathfind("ucontrol" "pathfind") = "Set the position for units to pathfind to"
        UCTargetP("ucontrol" "targetp")   = "Makes a unit shoot with velocity prediction"
        UCItemDrop("ucontrol" "itemDrop") = "Makes a unit drop items"
        UCMine("ucontrol" "mine")         = "Makes a unit mine a given coordinate"
    ---


    3i0o:
        ControlShootP("control shootp") = "Set where a turret should shoot with velocity prediction"

        UCApproach("ucontrol" "approach") = "Set the position for units to approach"
        UCTarget("ucontrol" "target")     = "Set the position for units to target"
        UCItemTake("ucontrol" "itemtake") = "Make a unit take items"
    ---

    4i0o:
        ControlShoot("control shoot") = "Set where a turret should shoot"
    ---

    1i1o:
        Set("set") = "Set variable"

        // Looks wrong, but isn't
        BlockLookup("lookup" "block")   = "Lookup a block by index"
        Unitookup("lookup" "unit")      = "Lookup a unit by index"
        ItemLookup("lookup" "item")     = "Lookup an item by index"
        LiquidLookup("lookup" "liquid") = "Lookup a liquid by index"
        TeamLookup("lookup" "team")     = "Lookup a team by index"

        GetLink("getlink") = "Get a link by index"
    ---

    2i1o:
        OpAdd("op" "add") = "Addition"
        OpSub("op" "sub") = "Subtraction"
        OpMul("op" "mul") = "Multiplication"
        OpDiv("op" "div") = "Division"
        OpExp("op" "pow") = "Exponentiation"

        OpIntDiv("op" "fdiv")  = "Integer division"
        OpMod("op" "mod")      = "Modulo"
        OpTrueMod("op" "emod") = "True modulo (gets sign from divisor)"

        OpEq("op" "equal")     = "Equality check"
        OpNot("op" "notEqual") = "Inequality check"

        OpRead("read")   = "Read from memory cell"
        OpWrite("write") = "Write to memory cell"
    ---

    4i1o:
        PackColour("packcolor") = "Pack a colour from RGBA values"
    ---
}
