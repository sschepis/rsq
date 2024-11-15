# Using the Quantum Math Library from JavaScript

## Setup

1. Build the WebAssembly module:
```bash
wasm-pack build --target web
```

2. Import in your JavaScript:
```javascript
import init, { 
    Complex,
    ComplexMatrix,
    QuantumMath,
    SystemMetrics
} from './pkg/rsq.js';

// Initialize the WASM module
await init();
```

## Accessing Constants

Constants are accessed through static methods of the QuantumMath class:

```javascript
// Get system constants
const classicalComplexity = QuantumMath.getClassicalComplexity();
const quantumComplexity = QuantumMath.getQuantumComplexity();
const successRate = QuantumMath.getSuccessRate();
const entanglementCorrelation = QuantumMath.getEntanglementCorrelation();

// Get threshold values
const phaseCoherence = QuantumMath.getPhaseCoherenceThreshold();
const entanglement = QuantumMath.getEntanglementThreshold();
const phaseAlignment = QuantumMath.getPhaseAlignmentThreshold();
const zeroProximity = QuantumMath.getZeroProximityThreshold();
```

## Basic Operations

### Creating Complex Numbers
```javascript
// Create a complex number (2 + 3i)
const complex = new Complex(2.0, 3.0);
```

### Working with Matrices
```javascript
// Create a 2x2 complex matrix
const matrix = new ComplexMatrix(2, 2);

// Set matrix elements
const value = new Complex(1.0, 0.0);
matrix.set(0, 0, value);
```

### Wave Function Operations
```javascript
// Create arrays for primes and gaps
const primes = new Float64Array([2, 3, 5, 7, 11]);
const gaps = new Float64Array([1, 2, 2, 4]);

// Compute wave function
const psi = QuantumMath.waveFunction(1.0, 0.5, 1.0, primes, gaps);
```

### Working with Zeta Zeros
```javascript
// Create an array of zeta zeros
const zeros = [
    { value: { real: 0.5, imag: 14.1347 } },
    { value: { real: 0.5, imag: 21.0220 } }
];

// Calculate zero proximity
const proximity = QuantumMath.zeroProximity(matrix, zeros);
```

## Complete Example

```javascript
import init, {
    Complex,
    ComplexMatrix,
    QuantumMath,
    SystemMetrics
} from './pkg/rsq.js';

async function runQuantumCalculations() {
    // Initialize WASM module
    await init();

    // Create quantum states
    const matrix1 = new ComplexMatrix(2, 2);
    const matrix2 = new ComplexMatrix(2, 2);

    // Set up initial states
    matrix1.set(0, 0, new Complex(1.0, 0.0));
    matrix2.set(0, 0, new Complex(0.0, 1.0));

    // Calculate various metrics
    const alignment = QuantumMath.phaseAlignment(matrix1, matrix2);
    const entanglement = QuantumMath.entanglementStrength(matrix1);

    // Create zeta state
    const zetaState = QuantumMath.zetaState(1.0, 0.5, 10.0);

    // Calculate zero proximity
    const zeros = [
        { value: { real: 0.5, imag: 14.1347 } },
        { value: { real: 0.5, imag: 21.0220 } }
    ];
    const proximity = await QuantumMath.zeroProximity(matrix1, zeros);

    // Create and use system metrics
    const metrics = new SystemMetrics();
    metrics.resonance_score = 0.95;
    metrics.entanglement_strength = 0.85;
    metrics.zeta_overlap = 0.90;
    metrics.protection_strength = 0.88;
    metrics.hash_stability = 0.92;
    metrics.interference_strength = 0.89;

    const optimizationScore = metrics.optimizationScore();

    return {
        phaseAlignment: alignment,
        entanglementStrength: entanglement,
        zeroProximity: proximity,
        optimizationScore: optimizationScore,
        constants: {
            classicalComplexity: QuantumMath.getClassicalComplexity(),
            quantumComplexity: QuantumMath.getQuantumComplexity(),
            successRate: QuantumMath.getSuccessRate(),
            thresholds: {
                phaseCoherence: QuantumMath.getPhaseCoherenceThreshold(),
                entanglement: QuantumMath.getEntanglementThreshold(),
                phaseAlignment: QuantumMath.getPhaseAlignmentThreshold(),
                zeroProximity: QuantumMath.getZeroProximityThreshold()
            }
        }
    };
}

// Usage
runQuantumCalculations().then(results => {
    console.log('Quantum Calculations Results:', results);
}).catch(error => {
    console.error('Error in quantum calculations:', error);
});
```

## Performance Considerations

1. **Memory Management**
   ```javascript
   let matrix = null;
   try {
       matrix = new ComplexMatrix(2, 2);
       // Use matrix...
   } finally {
       if (matrix) matrix.free();
   }
   ```

2. **Array Handling**
   - Use TypedArrays for real numbers
   - Format complex arrays correctly for zero_proximity:
   ```javascript
   const zeros = [
       { value: { real: x1, imag: y1 } },
       { value: { real: x2, imag: y2 } }
   ];
   ```

3. **Async Operations**
   ```javascript
   // In worker.js
   import init, { QuantumMath } from './pkg/rsq.js';
   
   self.onmessage = async function(e) {
       await init();
       const result = QuantumMath.waveFunction(e.data.x, e.data.t, e.data.sigma, e.data.primes, e.data.gaps);
       self.postMessage(result);
   };
   ```

## Error Handling

```javascript
try {
    const result = await QuantumMath.zeroProximity(matrix, zeros);
} catch (error) {
    console.error('Zero proximity calculation error:', error);
    // Handle error appropriately
}
```

## Building for Production

1. **Optimize for Size**
```bash
wasm-pack build --target web --release -- --features wee_alloc
```

2. **Optimize for Speed**
```bash
wasm-pack build --target web --release -- --features parallel
```

3. **Include in Web Project**
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Quantum Math Demo</title>
</head>
<body>
    <script type="module">
        import init, { QuantumMath } from './pkg/rsq.js';

        async function run() {
            await init();
            const complexity = QuantumMath.getClassicalComplexity();
            console.log('Classical Complexity:', complexity);
        }

        run();
    </script>
</body>
</html>
```

## Common Issues and Solutions

1. **"Cannot read property of undefined"**
   - Solution: Ensure you've awaited init()
   - Make sure all objects are properly constructed

2. **"Invalid zero array format"**
   - Solution: Format zero arrays correctly:
   ```javascript
   const zeros = [
       { value: { real: 0.5, imag: 14.1347 } }
   ];
   ```

3. **Performance Issues**
   - Use Web Workers for heavy calculations
   - Batch operations when possible
   - Properly manage memory with .free()
