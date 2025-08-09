# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
Jigs - A high-performance RISC-V runtime for ARM64 systems with gas-metered execution. The project aims to build a complete RISC-V execution environment that:
1. Decodes and encodes RISC-V 32-bit instructions
2. AOT-compiles RISC-V code to native ARM64 for near-native performance (compiles when loaded, not during execution)
3. Provides gas-metered execution for controlled resource usage in blockchain/sandboxed environments

Currently implements full decoding and encoding for all RV32IM instructions (base integer instructions plus M extension), with AOT compilation and gas tracking planned.

## Links
**@docs/DEVELOPMENT.md** - Development guidelines, commands, testing conventions, and contribution guidelines
**@docs/ARCHITECTURE.md** - Project architecture, module structure, and test organization
**@docs/ROADMAP.md** - Feature roadmap and project tracking with implementation status