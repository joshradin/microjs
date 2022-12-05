# MicroJs

## Goals and Features
- Provide an embeddable javascript engine for programs
- Insert data into engine and retrieve data
- Have C-bindings to allow for use outside of rust programs
- Support the latest version of ECMA script specification, [ECMA-262]
- Be smaller than the embeddable quickjs implementaiton
- Be 🚀 blazingly fast 🚀

## Design
There shall be several major components to the MicroJS implementation:
1. A parser to convert source code into an AST
2. A compiler that converts the AST into some bytecode
3. The engine which executes the bytecode in some manner

### Parser
The parser shall match the specification set forth by the ECMA. The only job for the parser is to
create the AST, and shall not perform any analysis of the generated AST.


[Implementation details](#parser-1)


## Implementation
These are details into the individual implementations of the different components of microjs.

### Parser
The parser should take in a fully generated `&str` value. It should be the job of the parser to directly
convert this `&str` value into an abstract syntax tree. The abstract syntax tree shall be defined by following
the grammar as closely as possible as defined by the [ECMA-262].

### Engine
The main engine of the microjs implementation should be bytecode based. 
1. Tree-walk
2. Bytecode

The engine should attempt to immediately execute the AST. In the background, associated bytecode should begin to be
produced for the AST. Once this is available for a given symbol, the engine should change execution mode into
the bytecode execution.


### Bytecode
The bytecode itself shall be based loosely on java.


<!--- links --->
[ECMA-262]: https://tc39.es/ecma262/