# bevy_oceansim
An experiment to use Bevy to simulate an ocean using compute shaders and FFT.

Based on the principles outlined by Tessendorf, Jerry. (2001). Simulating Ocean Water. SIG-GRAPH'99 Course Note (https://people.computing.clemson.edu/~jtessen/reports/papers_files/coursenotes2004.pdf). 

**Note**: This repository is very much experimental and incomplete.

The goal:
Compute a Gaussian noise texture on the CPU at startup which is then passed to the GPU, calculates initial simulation parameters, and then produces the results as a vertex shader as a factor of time per frame.
