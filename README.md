# Proxyz 

**Proxyz**, pronounced **/PROK-siss/**, is a low-level systems programming language designed for high-level syntax ergonomics and memory safety.

The current implementation (`pxzc-rust`) is the reference frontend pipeline, written in Rust to establish the grammar mechanics, AST structure, and execution rules before transitioning to a compiled backend.

---

## System Architecture

The compiler frontend is organized strictly by execution phases rather than enterprise layers. This minimizes pointer indirection and ensures a unidirectional data flow.

```
[ Raw Source String ] 
          │
          ▼  (Lexical Analysis Pass)
    Scanner::new()  ───► Emits: ScanResult<Vec<Token<'a>>>
          │
          ▼  (Syntactic Parsing Pass)
    Parser::new()   ───► Emits: ParseResult<'a, Vec<Stmt<'a>>>
          │
          ▼  (Runtime Execution Pass)
   Interpreter::new() ───► Evaluates Statements & Manages State via Environment
```

## Repository Layout

```
pxzc-rust [pxzc]
├── Cargo.toml
└── src/
    ├── main.rs
    ├── proxyz.rs
    ├── errors.rs        // Diagnostic error definitions
    ├── interpreter.rs   // Phase 3 [TEMP]: Tree-walk execution engine
    │   ├── mod.rs       
    │   ├── core.rs
    │   └── environment.rs
    ├── lexer/           // Phase 1: Stream Tokeniser
    │   ├── mod.rs  
    │   ├── literal.rs
    │   ├── scanner.rs
    │   ├── token.rs
    │   └── token_kind.rs
    └── parser/          // Phase 2: Grammar Engine
        ├── mod.rs    
        ├── ast_printer.rs   
        ├── core.rs
        ├── expr.rs
        └── stmt.rs
```

## Current Progress
- [x] Project Architecture & Repository Setup
- [x] Lexical Analysis
- [x] Abstract Syntax Tree (AST) & Parsing
- [x] Runtime Evaluation
- [ ] Static Type Analysis
- [ ] Compiled Backend Integration

## Getting Started
### Prerequisites

To build and run Proxyz, you require the standard Rust toolchain. If you do not have it installed, fetch it via rustup.rs:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation

Clone the repository and move into the project directory:

```
git clone https://github.com/asimadalat/proxyz.git
cd proxyz
```

### Running the Interactive REPL

To launch the interactive environment and evaluate expressions on the fly, run Cargo without any trailing file arguments:

```
cargo run
```

Once inside, you can type expressions or variable definitions using the 'log' directive:

```
> var num = 42
> log num + 8
50
```
