# kalc
[![AUR](https://img.shields.io/aur/version/kalc.svg)](https://aur.archlinux.org/packages/kalc/)

requires gnuplot for graphing, a modern terminal like windows terminal on windows

history file is stored in ``~/.config/kalc.history`` or ``C:\\Users\\%USERNAME%\\AppData\\Roaming\\kalc.history``

config file is stored in ``~/.config/kalc.config`` or ``C:\\Users\\%USERNAME%\\AppData\\Roaming\\kalc.config``

# usage
```
Usage: kalc [FLAGS] function_1 function_2 function_3...
FLAGS: --help (this message)
--tau fractions are shown in tau instead of pi
--deg compute in degrees, gets rid of complex support for non hyperbolic trig functions
--2d [num] number of points to graph in 2D
--3d [num] number of points to graph in 3D
--xr [min] [max] x range for graphing
--yr [min] [max] y range for graphing
--zr [min] [max] z range for graphing
--point [char] point style for graphing
--sci toggles scientific notation
--base [num] sets the number base (2,8,16)
--prompt toggles the prompt
--color toggles color
--debug displays computation time in nanoseconds

- Type "exit" to exit the program
- Type "clear" to clear the screen
- Type "help" to see this message
- Type "history" to see the history of calculations
- Type "deg" to switch to degrees mode
- Type "rad" to switch to radians mode
- Type "tau" to show fractions in tau
- Type "pi" to show fractions in pi
- Type "prompt" to toggle the prompt
- Type "color" to toggle color
- Type "2d=[num]" to set the number of points in 2D graphs
- Type "3d=[num]" to set the number of points in 3D graphs
- Type "xr=[min],[max]" to set the x range for graphing
- Type "yr=[min],[max]" to set the y range for graphing
- Type "zr=[min],[max]" to set the z range for graphing
- Type "point=[char]" to set the point style for graphing
- Type "sci" to toggle scientific notation
- Type "base=[num]" to set the number base (2,8,16)
- Type "_" to use the previous answer
- Type "a=[num]" to define a variable
- Type "f(x)=..." to define a function
- Type "f(x,y)=..." to define a 2 variable function
- Type "debug" toggles displaying computation time in nanoseconds

Trigonometric functions:
- sin, cos, tan, asin, acos, atan
- csc, sec, cot, acsc, asec, acot
- sinh, cosh, tanh, asinh, acosh, atanh
- csch, sech, coth, acsch, asech, acoth

Other functions:
- sqrt, cbrt, square, cube
- ln, log(base,num), root(base,exp)
- abs, sgn, arg
- ceil, floor, round, int, frac
- fact, subfact
- sinc, cis, exp
- deg(to_degrees), rad(to_radians)
- re(real part), im(imaginary part)
```
# example usage
```
kalc
> 1+1
2
> f(x)=sin(2x)
> f(x) //graphs f(x) in 2D
> f(pi/2) //evaluates f(x) at x=pi/2, so sin(2pi/2)=sin(pi)=0
0
> f(x,y)=x^2+y^2
> f(1,2) //evaluates f(x,y) at x=1, y=2, so 1^2+2^2=5
5
> f(x,y) //graphs f(x,y) in 3D
> a=3^3
> cbrt(a)
3
> im(exp(xi)) //graphs the imag part of exp(xi) in 2D, so sin(x)
```
# graphing
```
chars available for point style:
. - dot
+ - plus
x - cross
* - star
s - empty square
S - filled square
o - empty circle
O - filled circle
t - empty triangle
T - filled triangle
d - empty del (upside down triangle)
D - filled del (upside down triangle)
r - empty rhombus
R - filled rhombus
```