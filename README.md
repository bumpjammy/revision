# Obsidian Revision
## What does it do?
This is a quick rust program I made that allows you to revise your Obsidian/Markdown files. By passing in a directory, you can study the sections that you feel you need to.
## How do I use it?
To start, make a directory for your markdown files. For example, this will be `~/example/`<br>
You will want to make some markdown files. I recommend using obsidian for this!<br>
To mark down any keywords, enclose them in double asterisks, like \*\*this**<br>
Once you have made your files to revise, you can run the program.<br>
1) Download the latest version from the releases page or compile from source
2) Run the compiled program with your directory as an argument, for example:<br>
    `./revision ~/example/`
3) It will randomly select markdown files from the directory (and subdirectories!) and quiz you on the keywords

If you experience any issues with it running and not doing anything, ensure your markdown files include keywords!
## TODO/Planned features
- [ ] Confidence level for each file
- [ ] Different modes
- [ ] Refactor code