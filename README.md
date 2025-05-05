# Usage

## Building the tool

````
cargo build
````

Okay this is important now, after you build you will have target folder in root directory
of the tool, go inside it then you will have debug folder with untitled1.exe here in the /target/debug you have to drop used DLL for this case we are using Nezur.dll which is in src/bin/Nezur.dll, just drag it into debug folder,
And you are good to go! Have fun

### Running the tool
````
cargo run
````


You will be welcomed with Nezur CLI, you can see available commands using `help`
command okay, now it gets fun, run roblox and write and hit `attach`
command, after this check your roblox console, if it says `Initiated Nezur made by 1Cheats`
It means that it Initiaze() worked correctly and now you can execute

## Execution

You have to use

````
execute [Here example script but without any '' " " or something like this]
````

### Example executions

````
execute loadstring(game:HttpGet("https://raw.githubusercontent.com/EdgeIY/infiniteyield/master/source"))()
````

````
execute print("Hello World")
````



### You will get error if you try to execute with 

````
execute "print("hello world")"
````

There is no smart parser so use directly lua script after `execute`



## Support for other API's

For now i dont plan on adding more API's because i dont know much of them also, i would need to get exports of these API's
So they can be used in Wrapper
