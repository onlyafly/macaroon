# Macaroon Language Support for Visual Studio Code

This extension add Macaroon language support to VS Code.

## Features

* Syntax Coloring

## Installation

See [https://code.visualstudio.com/docs/extensions/publish-extension](https://code.visualstudio.com/docs/extensions/publish-extension) for more explanation.

### Preparation

Make sure you have [Node.js](https://nodejs.org/) installed. Then run:

```bash
npm install -g vsce
```

### Packaging extensions

You may want to package extensions without publishing them to the store. Extensions will always be packaged into a `.vsix` file. Here's how:

```bash
vsce package
```

This will package your extension into a `.vsix` file and place it in the current directory. It's possible to install `.vsix` files into Visual Studio Code.

## Credits

The macaroon2.tmLanguage.json file is modified from the Clojure syntax file included in VSCode:

[https://github.com/Microsoft/vscode/blob/master/extensions/clojure/syntaxes/clojure.tmLanguage.json](https://github.com/Microsoft/vscode/blob/master/extensions/clojure/syntaxes/clojure.tmLanguage.json)

The macaroon.tmLanguage file is modified from an earlier Scheme extension by Allen Huang.

[https://github.com/sjhuangx/vscode-scheme](https://github.com/sjhuangx/vscode-scheme)

That file originates from egrachev's sublime-scheme to enable syntax on vscode:

[https://github.com/egrachev/sublime-scheme/blob/master/Scheme.tmLanguage](https://github.com/egrachev/sublime-scheme/blob/master/Scheme.tmLanguage)
