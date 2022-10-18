# Bitcoin-Script-Interpreter
This will gradually be updated whenever I have time. 
Currently I am extremely busy with work and side job, so not lots of time to work on the development of this interpreter .
Expect the update of this readme to be very slowwww (for the time being)

On a high level, this interpreter will allow developers who use this tool to see what their Bytes (TBytes) are made up of. 
After performing numerous opcodes, it would be nice to see what the remaining Bytes in the stack are made from. 
This interpreter will show you what opcodes were performed to create the parent TByte.

We do this by a recursive data structure.
We have a Struct (Tbytes) that stores the data(Vec<u8>), parts(Vec<TBytes>), name(Option<String>)
This way when we analyze the parts, it will contain the TBytes that were used to produce the parent TByte, and annotate what opcodes were used (due to name).
I let name be an option as maybe the developer dosent want to have to name the function they use every time for simplicity purposes. 

Lets look at a simple example:

    TBytes 1 has [1,2,3]
    TBytes 2 has [4,5,6]
    the stack []
    
    Now we run the run_script function, putting in the opcodes that we want to perform
    run_script(the_stack, push TBytes 1, push TBytes 2, Op::Opcode(Opcode::OpCat)]).unwrap();
    
    Below is the result, lets evaulate it!
    
    [
    TBytes {
        data: [                   So the only TBytes on the stack now should be the OpCat TBytes, as we popped TBytes 1 and 2 off the stack.
            4,                    We can see that the data is 456123, which is the result of calling concat on TBytes 1 data and TBytes 2 data.
            5,                    
            6,
            1,
            2,
            3,
        ],
        parts: [
            TBytes {
                data: [           Here is where the magic happens. 
                    4,            We can see that parts is made up of two TBytes:
                    5,              
                    6,                  The one we called the impl concat on (Tbytes 2).
                ],
                parts: [],      
                function: Plain,
                name: None,     
            },
            TBytes {
                data: [
                    1,                  And TBytes 1.
                    2,
                    3,
                ],
                parts: [],
                function: Plain,
                name: None,
            },
        ],
        function: Opcode(       The Opcode that was performed
            OpCat,
        ),
        name: Some(
            "Concat",             Here we can see that the TBytes name is Concat (we could of named this anything)
        ),                        letting the user know what function made that TBytes!
    },
]
    


I am to make a GUI for this eventually, and alot more positive changes. 
This is just the beggining.

