# This is a quick wallpaper generator from the first JWST images.

The application takes at least two parameters: the width and height of the image
to be generated and optionally the name of the output file. If no output file is
specified, the output is written to wp.png in the current directory.

The command chooses a random image from the JWST in the wallpaper directory, a
random zoom level according to the resolution of the original image and the 
requested image size, and then crops a random portion of the originale image to make a kind of new wallpaper each time.

Syntax: jwst width height [output file]

Have fun!
