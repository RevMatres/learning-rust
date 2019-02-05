
-- example starting point
c = (-0.25, -0.25)

-- mandelbrot(z, c)
mandelbrot (a,b) (r,i) = (a + (r*r - i*i), b + (r*i + r*i))

-- the mandelbrot sequence starting at c = (a,b)
mandelseq (a,b) = iterate (mandelbrot (a,b)) (0.0,0.0)
