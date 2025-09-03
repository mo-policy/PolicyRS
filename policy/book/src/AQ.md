# Anticipated Questions

1. Should I invest the time to learn Policy Language?

    Policy Language is new and, as of June 2025, there are no adopters of the language, there is no stable interpreter, and no development tool integration. So, probably not. However, if you're a Rust programmer it would be fairly easy to clone the repository, run 'cargo build' and 'cargo test' to confirm the interpreter runs for you. Then copy and modify a test that is working to experiment with the language. If you're not a Rust programmer, you could look through the documentation to get a feel for the language, especially these anticipated questions.

2. I'm into programming languages and computing theory, what's a concise description of Policy Language?

    Policy Language is a rewrite system expressed using the Decentralized Identity (DID) data model, though there is also a parser of a text format similar to OCAML/F# for ease of programming. Policy Language has the basics of an untyped lambda calculus which is used to create values within the DID data model. Also, the send and receive expressions function as their counterparts in the pi calculus. Send is asynchronous, and receive can be gated by a list of pattern matching rules evaluated against incoming messages. 

    The language allows for concurrent semantics but executes in a single thread and with a fixed, top down, depth first order. Expressions can be blocked, for example a receive expression waiting for a message on a channel. Blocked expressions are skipped over and the next expression in a parallel expression is executed, if there is one. Rewrites happen in place. Any program which is not fully reduced, because some portion of it is blocked, is expected to be tried again, from the top, when conditions might allow it to reduce further. 

    Any value which complies with the DID data model is a valid Policy program, however there are special types of values, called sorts, which are interpreted as code, not data. These sorts, and their related rewrite rules, define the language. Any value which is not a sort is treated as a constant. Policy Language has a novel (I think) feature of allowing for pattern matching on the sort about to be executed. There is a sort, called the policy sort, which defines pattern matching rules to be in effect during the evaluation of an inner expression. This allows for conditional branching based on the code about to execute. It's expected use is when receiving code over a channel and then evaluating that code within the bounds of some policy rules. For example, the policy might constrain the code to only send and receive over certain channels.

3. How is Policy Language related to Decentralized Identity technologies?

    * Policy Language is designed for identity computing.
    * Any valid DID document is a valid Policy Language program, though only certain values, called sorts, are treated as code. Any other data is treated as a constant.
    * The send and receive expressions are meant to work with DID Comm messaging, though not strictly constrained by this protocol.
    * DID resolution acts as the module resolution mechanism and is the primary means of modularization.
    * DID URL resolution semantics are meaningful to Policy language evaluation in that query and value pairs map to name bindings and fragments can be used to reference values in other Policy programs.

4. How is Policy Language related to digital wallets?

    Digital wallets could host a Policy Language interpreter in the same way web browsers host a JavaScript interpreter. As of June 2025, there are no digital wallets hosting Policy Language.

    To understand why Policy Language would be useful within a digital wallet, you need to understand that the receive expression, a part of Policy Language, is effectively a server expressed in code. You could run any number of these servers within your wallet without needing to manage a computer. You would simply install a Policy app which encodes the rules for interacting with clients. For example, suppose you wish to offer something you produce for sale to potential buyers. The Policy app would contain the rules for interacting with buyers, with orders and payments appearing in your wallet. There are countless additional scenarios. 

5. What other languages is Policy Language similar to?

    Most importantly, Policy Language is a rewrite system, not a compiled language, and maybe not even an interpreted language though I'm not sure there's a strict definition for what an interpreted language is. 

    Another rewrite language you likely know is algebra. The stuff you learned in school. You start with a complex expression and go through a series of rewrite rules which produce equivalent expressions with reduced complexity. For example: 
    
        1 + 2 * 3
        = 1 + 6
        = 7

    Policy Language has certain values that can be reduced. For example:

        if 1 + 2 * 3 > 5 then "bigger than 5" else "smaller than 5"
        = if 1 + 6 > 5 then "bigger than 5" else "smaller than 5"
        = if 7 > 5 then "bigger than 5" else "smaller than 5"
        = if true then "bigger than 5" else "smaller than 5"
        = "bigger than 5"

    Note the example above is in PolicyML, a text based representation of Policy Language. The 'real' representation is in the DID data model but is very verbose. To get a feel for that format, here's the last two lines of the above example in the DID data model. (Below is really a JSON representation of the Policy Language DID data.)

        { 
            "$sort": "If",
            "condition": true,
            "then": "bigger than 5",
            "else": "smaller than 5"
        }

        = "bigger than 5"

    Oddly enough, the language used in OpenSCAD is similar to Policy Language, though the result of an OpenSCAD program is a 3D drawing, not a DID Document, but the concepts are surprisingly similar. To get a feel for the subtle differences between a more common programming experience versus a rewrite system, OpenSCAD is a fully functional system with good documentation you might try. 

    Also,  spreadsheet programs are similar to Policy Language, though Policy Language produces DID documents, not tables of data, and Policy Language reduces in-place where as spreadsheets retain their code expressions as the programmer wrote them, keeping the data and the code. Policy Language will probably have this feature someday, but for now it reduces and overwrites the executing program. This choice to update in-place makes more sense when considering the hashing of Policy Programs. Policy programs are identified by their hashes (in the DID for the program, e.g., did:policy:12345abcd) so any version of a Policy program is always identifiable.

5. What works in the interpreter, what doesn't, and what's planned?

    There are two implementations of a Policy Language intepreter. One in JavaScript (PolicyJS) and one in Rust (PolicyRS). PolicyRS is the most recent and reflects the current (as of June 2025) design of Policy Language. In time PolicyJS should be brought up-to-date with PolicyRS, but for now PolicyJS is not being actively developed.

    Here are the major components of the PolicyRS intpreter and their status as of June 2025.

    * Rewrite

        The rewrite component is the heart of the interpreter. It performs the depth first traversal of a Policy program, detecting values which are sorts and performing the appropriate rewrite function. Rewrites are substantially complete for every sort, with the exception of looping constucts. There are tests for normal execution of each sort, and a blocked execution. The tests and implementation serve as a specification for Policy Language, also documented here. If there's a conflict between the PolicyRS rewrite implementation and the documentation, the PolicyRS implementation is likely more true to the intended design.

        There's a PolicyML parser implemented using the Rust parser and lexer tools of LALRPOP and Logos. These work well enough to write simple Policy programs but are not robust. There are no error messages and syntax precedence is not refined. You'll need to experiment with wrapping expressions in parenthesis to get a correct or successful parse.

        What's not complete? 

            * Errors. There's no consi.stent design or implentation for errors or exceptions. There's no consistent treatment of type checking

            * Pattern Matching. Pattern matching is only implemented for a few sorts, such as Constant, Lookup, and the container types of Map, List, Set, and Tuple. There are very few tests for pattern matching.

        What's planned?

            * Retainer Metering. It's planned to implement a cost for every action of the interpreter, called a retainer (similar to gas). The specifics are not designed, but likely handling each node will have a unit cost. For example reducing a list of three constants would have a four unit cost, one for each list element and one for the list itself. Running out of retainer would produce a blocked result and terminate the subsequent execution with a positive retainer ballance.

            * Normalization of Hashing. Rust hashes are u64 which affect the meaning of Policy comparison. At the same time, Policy DIDs will use cryptograhic hashes, which are likely going to be open to specification within the Policy DID document. These need to agree, and this is likely inconsistent today. Also, the retainer cost system will likely charge a unit fee for each node traversal, so using hashing to avoid traversals will benefit the user. The current implementation is not consistent across the areas which would perform a traversal of the nodes.

            
6. What is meant by 'blocked'?
