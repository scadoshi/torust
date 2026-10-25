# General
    - 3D coordinates translate to a 2D plane by keeping X and Y the same. Z becomes brightness
    - Actual Y gets translated to smaller Y' at screen via some kind of focal length? 
    - Does X change as well? 
        - Yes it does
    - Some algs
        - X, Y and Z after transformation into X', Y' and Z' must maintain ratio/relationship so X/Z = X'/Z'
    - On "we can re-use z-1 when computing x' and y': Dividing once and multiplying by z-1 twice is cheaper than dividing by z twice" ... Why? Why is it cheaper to divide by 1/z than z? Because it is a smaller number by definition?

# Constants
## Basic
    - z is depth of real point from viewer
    - z' is depth of screen from viewer. It's a given so we call it a constant: K1
    - K2 is a constant value we add to z to push donut away from viewer
    - x and y just remain x and y: the true horizontal and vertical positioning of the coordinates
    - x' and y' are the translated coordinates on screen

## Radii
    - R1 is radius of internal circle rotated around axis to make donut
    - R2 is radius of that axis

## Axes to rotate around
    - A and B
