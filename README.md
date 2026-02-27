# 2D Ising Model Simulation

A high-performance Rust implementation of the 2D Ising model using the Metropolis-Hastings algorithm. This project demonstrates various optimization techniques for computational physics simulations.

## Table of Contents
- [Overview](#overview)
- [Key Features](#key-features)
- [Optimizations](#optimizations)
- [Results](#results)
- [Installation & Usage](#installation--usage)

## Overview

The Ising model is a mathematical model of ferromagnetism in statistical mechanics. The system consists of discrete variables called spins that can be in one of two states (+1 or -1) and are arranged in a graph, usually a lattice.

This simulation explores the phase transition between disordered and ordered states as a function of temperature.

## Key Features

- **Metropolis Algorithm**: Standard Markov Chain Monte Carlo (MCMC) method for sampling the equilibrium state.
- **Periodic Boundary Conditions**: Simulated on a toroidal topology to minimize edge effects.
- **Adaptive Temperature Stepping**: Dense sampling near the critical temperature ($T_c \approx 2.269$) to accurately capture phase transitions.
- **Statistical Analysis**: Built-in calculation of average magnetization and standard deviation.

## Optimizations

This project implements several performance-oriented optimizations:

1.  **Memory Layout**: Grid is stored in a flat 1D vector (`Vec<i8>`) in row-major order to maximize CPU cache hit rates and provide better data locality.
2.  **Bitwise Arithmetic**: Boundary conditions are handled using bit-masking (`& mask`) instead of the expensive modulo operator. This optimization requires the grid size ($N$) to be a power of two.
3.  **Metropolis Lookup Tables**: Boltzmann factors for the energy changes ($\Delta E \in \{4, 8\}$) are precomputed and stored in a lookup table, eliminating redundant calls to the `exp()` function in the simulation hot loop.
4.  **Inlining**: Critical functions like index calculation are hinted for aggressive inlining to reduce call overhead.

## Results

### Performance Benchmark vs Real Simulation Benchmark

Putting these all optimizations together the benchmark results for a 2048x2048 grid, temperature 2.269 and number of sweeps equals 100 on a modern CPU are:

| implementation | avg time [s] | std deviation [s] | optimization factor |
| :--- | :--- | :--- |
| **Classic** | 14.3851 | 0.4591 | 1.0x |
| **Optimized** | 7.3398 | 0.2201 | **~1.96x** |

For the real simulation for a 1024x1024 grid, temperature from 1.0 to 4.0 and number of sweeps equals 500 results are:

| implementation | avg time [s] | optimization factor |
| :--- | :--- | :--- |
| **Classic** | 4413.16 | 1.0x |
| **Optimized** | 2381.20 | **~1.85x** |

### Magnetization vs. Temperature
The simulation clearly shows the phase transition at the critical point. Below $T_c$, the system remains ordered (high magnetization), while above $T_c$, it becomes disordered (near-zero magnetization).

![Ising Classic Magnetization](ising_classic_plot.png)
*Figure 1: Magnetization vs. Temperature (Classic Implementation)*

![Ising Optimized Magnetization](ising_optimized_plot.png)
*Figure 2: Magnetization vs. Temperature (Optimized Implementation)*
