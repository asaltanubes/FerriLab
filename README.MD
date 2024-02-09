# Welcome to FerriLab!

This library allows you to process data from a physics laboratory, make
calculations between measures and plot figures.

## Usage

`FerriLab` is based in the object [Measure], used to store either
the value and the error of a measure. To make it easy to create a new measure
the [measure] macro let you introduce data on various formats.

The object measure has all operations implemented between measures and numbers
along with the basic math functions you may need for processing your data.

With every operation applied to a measure, the error will be modified following
error propagation. Also a measure can be aproximated to the first significative
figure of the error.

```rust
use ferrilab::{measure, Measure};
let time = measure!([0.227, 0.312, 0.4019, 0.512], [0.012, 0.023, 0.025, 0.048]);
let position = measure!([2.425, 3.41515, 5.13545, 7.24524], [0.2, 0.43, 0.544, 0.872]; true);
let speed = &position / &time; // Using reference allows position and time to still be used.
let angle_grads = measure!([0, 30, 45, 60, 90, 180], [0.01, 3, 4, 5.8, 7, 11.6]);

let angle_rad = angle_grads.rad(); // Transform grades to radians.
let cosine = angle_rad.cos(); // Calculates the cosine of angles.
```
