sgpasswd
-------------
The **s**uckless **g**enerator of **passw**or**d**s is a memory safe, suckless utility made by me, that aims to make it easier to generate a password from the commandline instead of typing 5m pipes just to get the result from /dev/urandom. Not only it gets from /dev/urandom from /dev/random. It also uses base64 to encode it if you pass the --base64 option. It can also do it in hex with --hex. It can also check your password strength: --check or --checkp <password>. However, even though it is suckless, it doesn't follow the UNIX philosophy of one object has only one purpose. But this tool doesn't abide by that rule. So you may say that sgpasswd is a meta tool made by me that makes your life easier.

Requirements
-------------
In order to build, and install sfetch, you may install any of the following compilers:
- rustc (Rust C Compiler)
- cargo (The Compiler and Package Manager of Rust)

In order to change the compiler being used on the Makefile, change it through the config.mk file.

Installation
-------------
Edit config.mk to match your local setup (sfetch is installed into
the /usr/local/bin namespace by default).

Afterwards enter the following command in order to build and install sfetch, then, clean the compiled binary. (if
necessary, as root):

    make install clean


Usage
-------------
Use /usr/local/bin/sfetch or use the $PATH variable: 
```
# Full path:
/usr/local/bin/sfetch

# $PATH variable:
sgpasswd <args> <password>
```
Credits
-------------
It's not based on anything, its purely independent on Rust, it's crates: Base64 and Hex. And mostly on /dev/random and /dev/urandom.
README based on the suckless terminal's README.md.
