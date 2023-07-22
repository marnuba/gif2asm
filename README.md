# gif2code

This is a tool to convert a gif image into data that can easily be used in common 8-bit assemblers. The main target platform is the Foenix family of computers, but it might be useful for other vinatage and retro systems as well.

These systems use a 256 color palette from the full 24 bit color range. Incidentially, the gif format also supports 256 colors and is available in most common graphic programs.

Other tools are available to convert images as well (e.g. FoenixIDE). This tool is mainly aimed to be used in an automatic build process that does not need user interaction (i.e. from a script or batch file), so development cycles in the design of a program might be shortend (save gif to project directory, run build script, run program with updated graphics - all in one command).

## Usage

gif2code <INPUT.GIF>

This produces an INPUT.asm file that can be included by the given assembler.

gif2code --bin <INPUT.GIF>

This produces two files: INPUT.clut and INPUT.bin. These files can be used from an assembler by including them as binary files.

The format of the files is as follows:

### LUT File

The first byte represents the number of entries. The value "00" represents 256 colors, the maximum.

The entries follow at offest 1. Each entry consists of 4 bytes, just as the LUT in the Foenix computers:
The first three bytes represent the color value in the order B, G, R.
The fourth byte is reserved for later usage.

### IMG File

The first 4 bytes represent the width and height of the image data, lo-byte first.

Starting with the 5th byte, each byte represents one pixel. The value is the index of the color in the lookup table. The order is line by line from top to bottom, each line from left to right.




