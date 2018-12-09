TODO
----

*   use egg-mode from crates.io as soon as its updated to 0.13
*   more fractal types
    *   L systems <https://en.wikipedia.org/wiki/L-system>
        *   randomized L systems
    *   fractal flame <https://en.wikipedia.org/wiki/Fractal_flame>
        *   use <http://sprott.physics.wisc.edu/pubs/paper210.pdf>
            to create more interesting IFS and flames
    *   strange attractors
    *   diffusion limited aggregation
    *   self-avoiding walk
    *   dragon curve
    *   menger sponge
    *   3D fractals (raycasting? shader programming?)
        *   e.g. <http://blog.hvidtfeldts.net/index.php/2012/09/rendering-3d-fractals-without-a-distance-estimator/>
        *   e.g. <http://www.diva-portal.org/smash/get/diva2:325566/FULLTEXT01.pdf>

*   random default vibrancy value
*   parse function from cla
*   neural net classifier: interesting <-> not interesting
*   vibrancy and gamma need to be saved in the json export
*   output rpn as infix notation

*   Generic L-systems:
    -   pass rules per cli
    -   random generation of rules
    -   pass specific systems as rule-strings instead of the current overheady way

*   wait for rand to enable arbitrary hashable values as seed, then refactor
