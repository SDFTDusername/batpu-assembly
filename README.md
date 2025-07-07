# batpu-assembly
batpu-assembly is a [Rust](https://www.rust-lang.org/) library for doing any kind of work with the [BatPU](https://github.com/mattbatwings/BatPU-2).

It has enums for instructions and helper functions to assemble and disassemble.

It also contains some other things such as ``Location::Offset`` for my [batpu-assembler](https://github.com/SDFTDusername/batpu-assembler).

```rust
let mut instructions: InstructionVec = Vec::new();
let labels: Labels = HashMap::new();

instructions.push(
    // Load 5 into r1
    Instruction::LoadImmediate(
        Register::new(1).unwrap(),
        Immediate::new(5)
    )
);

let machine_code = instructions_to_binary(&instructions, &labels).unwrap();
```