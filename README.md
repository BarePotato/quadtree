Currently shifting this to a library.  
This entails not leaning on SFML's provided rect/point/vector functionality and instead creating our own.  
The parts already in use have been switched to remain funcitonal, namely Rect and Vector2.  
See issues for outstanding things.

# quadtree - below is still valid until it is removed from the readme as for running main.rs

- SFML 2.5 and CSFML 2.5 must be installed on your computer. You can download them here:

     - SFML 2.5: <https://www.sfml-dev.org/download/sfml/2.5.1/
     - CSFML 2.5: <http://www.sfml-dev.org/download/csfml/>

- On Windows make sure you include the .DLL's for the modules you are using in your target directory next to the created executable.
