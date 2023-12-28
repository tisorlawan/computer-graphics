# Raytracing

Raytracing is basically this algorithm:
```
Place the eye and the frame as desired
For each square on the canvas
    Determine which square on the grid corresponds to this square on the canvas
    Determine the color seen through that grid square
    Paint the square with that color
```

## Assumptions

- Fixed camera position, `O`
  O = (0,0,0)
- Fixed camera orientation, x+, y+ and z+
- Frame, with distance `d`, zentered on z axes, and parallel to x and y axes.
  We call this `viewport`. We draw on the canvas whatever we see through the viewport.

```
Place the camera and the viewport as desired
For each pixel on the canvas
    1. Determine which square on the viewport corresponds to this pixel
    2. Determine the color seen through that square
    3. Paint the pixel with that color
```
