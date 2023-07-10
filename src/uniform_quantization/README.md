# Uniform Quantization

### Example
```rs
use cqual::{UniformQuantizer, Quantize, QuantizeImage};

fn main() {
    let mut quantizer = UniformQuantizer::new(3, 3, 2);

    quantizer
        .quantize_image("imput.jpg", "output.jpg")
        .unwrap();
}
```

**References**
- https://web.cs.wpi.edu/~matt/courses/cs563/talks/color_quant/CQindex.html