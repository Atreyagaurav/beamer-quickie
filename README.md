# Beamer Quickie

Quickly open a beamer presentation file (.tex) and copy some slides.

# Introduction
This is a really simple app that I made to learn GTK in rust. 

The concept is to open LaTeX Beamer files, and see the corresponding pages from the compiled PDF along with your code, so that you can copy some slides from there without having to search the text manually.

The frame images are loaded from PDF of the same name as the LaTeX file (if present). if SyncTeX file is present, the pages will be accurate (as long as there is only one source TeX file), if SyncTeX is not present, the app will try to guess the pages for the frames (won't work if you have frames with `allowbreaks`, please don't).

![Screenshot of the GUI](screenshot.png "Screenshot showing an opened tex file with thumbnains of slides and some slides marked for export")

Use Generate to copy the selected slide's texts into the Editor, and then use Copy button to copy it (you can edit there if you want). The syntax highlighting is basic.

# Other Features
There are many features that could be possible with this. So far my knowledge of the GTK limits a lot of them. Feel free to make issues and maybe pull requests if that feature seems like a good idea.

One thing I wanted to do was save the file, but currently the TeX file's contents are read only for the `frame` environment, so anything between the frames is lost. Like `\section` and other environment info. Hence I didn't add a save button that might make you lose contents on the file.


