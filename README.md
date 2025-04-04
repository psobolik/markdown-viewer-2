# Markdown Viewer
Use this app to display a formatted Markdown file in desktop window. 

When the app has been installed, it can be run from the command line with the file to view as a parameter.

If it is installed as the file-type handler for the "text/markdown" mime type, you can also use it to open Markdown files from the file explorer.

## File-type Handler (Linux)
After installing the app, run the following command to associate Markdown files with it:
```shell
$ xdg-mime default markdown-viewer.desktop "text/markdown" 
```