# CLAUDE.md

## Project Overview
Jigs - A high-performance RISC-V runtime for ARM64 systems with gas-metered execution. The primary goal is to execute RISC-V programs on ARM64 machines at near-native speeds through AOT (Ahead-Of-Time) compilation.

Key features:
1. **AOT compilation**: RISC-V code is compiled to native ARM64 when loaded (not during execution) for maximum performance
2. **Gas-metered execution**: Controlled resource usage for blockchain and sandboxed environments
3. **Full RV32IM support**: Complete implementation of base integer instructions plus M extension

Currently, the decoding/encoding layer is complete for all RV32IM instructions, with the AOT compiler and runtime being the next major milestones.

## Links
**@docs/DEVELOPMENT.md** - Development guidelines, commands, testing conventions, and contribution guidelines
**@docs/ARCHITECTURE.md** - Project architecture, module structure, and test organization
**@docs/ROADMAP.md** - Feature roadmap and project tracking with implementation status