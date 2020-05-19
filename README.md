# cli-bin

The Common Loop Interface (CLI) project brings best practices from the [golden
age][CGI] of web computing to the world of batch processing.

## FAQ

### How does it work?

cli-bin reads a list of data and, for each entry, executes a program.  This
program is sometimes known as a _cli-bin script_.  The data may be homogenoeus
or heterogeneous and can be specified in myriad<sup>†</sup> formats.  No matter
the format, the data is passed to the script in a uniform fashion, by
environment variables.  This simple and boring approach is modeled after the
most famous way of writing a web application in 1998: [CGI][], the Common
Gateway Interface.

†: right now, just JSON

### What is it good for?

Quick and dirty semi-automation.

## License

MIT License, copyright Tom Jakubowski <tom@crystae.net>,
https://github.com/tomjakubowski.  See the [LICENSE](LICENSE) file.

[CGI]: <https://en.wikipedia.org/wiki/Common_Gateway_Interface>
