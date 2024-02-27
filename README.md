# poldot
Terminal tool for launch all your terminal scripts. The problems we want to solve are:
* Execute your scripts in a fast way.
* Avoid to remember the path of your scripts.
* Avoid to load your scripts in your terminal every time you open it.
* Create a documentation reader that asks you the script params and execute the script with the given params.
* Have different contexts for your scripts, if you have different projects with different scripts, you can configure whose scripts do you want to have in poldot.
* A fuzzy search thanks to [fzf](https://github.com/junegunn/fzf?tab=readme-ov-file).
* The scripts should be organized by the contexts (personal, work, project1, project2, etc.). And inside each context, the scripts should be organized by the type of script (git, docker, k8s, etc.).


## Install in dev mode
At the moment, it can be run in dev mode. It's only working for MacOS and Linux, the steps are:
* Clone the repository
* Install [Rust](https://www.rust-lang.org/tools/install)
* Install [fzf](https://github.com/junegunn/fzf?tab=readme-ov-file#installation)
* Have your $HOME/bin in your $PATH
* Execute in the root of the repository `make update-realease`
* Prepare your config file (see below)

After this steps you are ready to execute the command `poldot` in your terminal.

### Config file
The config file is a json file that should be located in your `$HOME/.config/poldot/config.json`. The file should have the following structure:
```json
{
    "directories": [
        {
            "alias": "context1",
            "path": "/path/to/your/first/context/"
        },
        {
            "alias": "script2",
            "path": "/path/to/your/second/context/"
        }
    ]
}
```