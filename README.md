# RSQ - Quantum Computing Library for Blockchain Mining

A WebAssembly-powered quantum computing library designed for optimizing blockchain mining operations through quantum mechanical principles.

## Features

- Quantum wave function manipulation
- Prime number resonance patterns
- Quantum interference optimization
- Entanglement-based mining enhancement
- Zeta function regularization
- Surface code error correction
- Quantum state protection

## Installation

```bash
npm install @sschepis/rsq
```

## Usage

### Basic Setup

```javascript
import init, { 
    QuantumMath,
    Complex,
    ComplexMatrix,
    SystemMetrics
} from '@sschepis/rsq';

// Initialize the WASM module
await init();
```

### Quantum Constants

```javascript
// Get system constants
const classicalComplexity = QuantumMath.getClassicalComplexity();
const quantumComplexity = QuantumMath.getQuantumComplexity();
const successRate = QuantumMath.getSuccessRate();
```

### Wave Functions

```javascript
// Create arrays for primes and gaps
const primes = new Float64Array([2, 3, 5, 7, 11]);
const gaps = new Float64Array([1, 2, 2, 4]);

// Compute wave function
const psi = QuantumMath.waveFunction(1.0, 0.5, 1.0, primes, gaps);
```

### Quantum States

```javascript
// Create a quantum state
const state = QuantumMath.zetaState(1.0, 0.5, 10.0);

// Calculate entanglement strength
const entanglement = QuantumMath.entanglementStrength(state);
```

### System Metrics

```javascript
const metrics = new SystemMetrics();
metrics.resonance_score = 0.95;
metrics.entanglement_strength = 0.85;
metrics.zeta_overlap = 0.90;

const score = metrics.optimizationScore();
```

## Complete Example

```javascript
import init, {
    QuantumMath,
    Complex,
    ComplexMatrix,
    SystemMetrics
} from '@sschepis/rsq';

async function runQuantumCalculations() {
    await init();

    // Create quantum states
    const matrix1 = new ComplexMatrix(2, 2);
    const matrix2 = new ComplexMatrix(2, 2);

    matrix1.set(0, 0, new Complex(1.0, 0.0));
    matrix2.set(0, 0, new Complex(0.0, 1.0));

    // Calculate metrics
    const alignment = QuantumMath.phaseAlignment(matrix1, matrix2);
    const entanglement = QuantumMath.entanglementStrength(matrix1);

    // Calculate zero proximity
    const zeros = [
        { value: { real: 0.5, imag: 14.1347 } },
        { value: { real: 0.5, imag: 21.0220 } }
    ];
    const proximity = await QuantumMath.zeroProximity(matrix1, zeros);

    return {
        phaseAlignment: alignment,
        entanglementStrength: entanglement,
        zeroProximity: proximity
    };
}
```

## Performance Optimization

### Memory Management

```javascript
let matrix = null;
try {
    matrix = new ComplexMatrix(2, 2);
    // Use matrix...
} finally {
    if (matrix) matrix.free();
}
```

### Web Workers

```javascript
// worker.js
import init, { QuantumMath } from '@sschepis/rsq';

self.onmessage = async function(e) {
    await init();
    const result = QuantumMath.waveFunction(
        e.data.x,
        e.data.t,
        e.data.sigma,
        e.data.primes,
        e.data.gaps
    );
    self.postMessage(result);
};
```

## API Documentation

For detailed API documentation, see [MATH.md](./src/quantum/MATH.md).

## Building from Source

1. Install Rust and wasm-pack:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack
```

2. Build the package:
```bash
npm run build
```

## Testing

```bash
npm test
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Theoretical Background

For detailed information about the quantum mechanical principles used in this library, see [theory.md](./theory.md).

## Citation

If you use this library in your research, please cite:

```bibtex
@software{rsq2023,
  author = {Schepis, Sebastian},
  title = {RSQ: Quantum Computing Library for Blockchain Mining},
  year = {2023},
  publisher = {GitHub},
  url = {https://github.com/sschepis/rsq}
}
