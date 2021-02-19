# reswmm Hydraulic Modeling Engine

Custom implementation of the EPA-Stormwater Management Model (EPASWMM), with some extra features.

## Purpose

Personal project to better understand hydraulic modeling and software development.

Potentially improve the existing EPASWMM engine by  leveraging a modular software architecture as well as the memory safety and concurrency features of Rust.

## Features

### Analysis

- Steady State
- Unsteady State
    - Kinematic Wave
    - Dynamic Wave

### Scenario Management

Separate system model (pipes, junctions, pumps, etc.)
from analysis inputs (rainfall, fixed inflows, fixed water surface elevations)
to analyze multiple scenarios with the same model.

### File Format

Use relational database for model data storage.

Potentially provide interface for arbitrary storage formats to run with the same analysis engine.

## Modules

### reswmm-core

The analysis enginer responsible for:
- Model creation/instatiation
- Simulation
- Results

### reswmm-io (not yet implemented)

Interface module responsible for:
- Reading inputs from file (or other source)
- Writing results to output (file or custom interface such as GUI)

### reswmm-cli

Commandline interface for providing inputs, outputs, and model parameters.

### furlong

Compile-time (i.e. zero-overhead) units library for enforcing type safety during development and allowing user to provide inputs in whatever units are most convenient.

At time of development, the rust library uom ("units of measure") was available, but compile times were not conducive to iterative development.

Furlong should eventually become its own external library, but being developed for reswmm.
