# Clock Offset Estimator: README

## Overview

This project implements a clock offset estimator based on the method described in the paper "A new model-based clock-offset approximation over IP networks". It's designed to estimate the time difference (offset) between two networked devices using one-way delay (OWD) measurements.

## Usage

To use this estimator, you need to provide a series of OWD measurements between two networked devices. The program will process these measurements and output the estimated clock offset in nanosecond precision.

Example usage:

```rust
let owd_measurements = vec![0.340, 0.360, 0.350, ...];
let offset = estimate(owd_measurements);
println!("Estimated clock offset: {}", offset);
```

## Contributing

Contributions are welcome! Please submit pull requests for any enhancements, bug fixes, or improvements.

## License

[MIT License](https://mit-license.org/)

## References

- E. Mota-Garcia, R. Hasimoto-Beltran, "A new model-based clock-offset approximation over IP networks", Computer Communications, Volume 53, 2014.
- D. H. Lehmer, "Mathematical methods in large-scale computing units", Annals of the Computation Laboratory, Harvard Univ., 1951.
- G. Marsaglia, "Generating a Variable from the Tail of the Normal Distribution", Technometrics, 1964.
- G. Marsaglia, W. W. Tsang, "A Simple Method for Generating Gamma Variables", ACM Transactions on Mathematical Software, 2000.

## Methodology Details: Constraints on Rho (ρ)

In the implementation of the clock offset estimation algorithm, the paper "A new model-based clock-offset approximation over IP networks" refers to a key parameter of the Gamma distribution as rho (ρ). However, in this codebase, the parameter is named 'alpha' for no specific reason, and this naming convention has been retained.

### Clarification on Alpha (in code) and Rho (ρ) (in paper)

- **Rho (ρ) in Paper**: The paper discusses a parameter, rho (ρ), which is crucial for the Gamma distribution used in modeling network delays.
- **Alpha in Code**: In our implementation, this parameter is referred to as 'alpha'. It's important to note that 'alpha' in the code corresponds to rho (ρ) as described in the paper.

### Constraints on Rho (ρ)

The paper recommends constraining the value of rho (ρ) between 1.0 and 4.0 for the following reasons:

1. **Model Accuracy**: The range likely represents a balance between modeling accuracy and computational complexity, based on empirical observations.
2. **Avoiding Extremes**: Constraining rho (ρ) helps avoid distributions that are too skewed or narrow, which may not accurately represent typical network delays.

This constraint is a critical aspect of the methodology and reflects the empirical observations and statistical considerations made in the paper. In our code, when we mention 'alpha', it is essentially the rho (ρ) parameter from the paper, constrained within the suggested range of 1.0 to 4.0.

## Acknowledgments

This project is inspired by the research and methodologies developed by experts in the field of network communications and statistical methods.
