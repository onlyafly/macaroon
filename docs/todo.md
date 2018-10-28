# Todo

How to deal with pointer problem: Make most references to Node point to a Rc<RefCell<Node>> "SmartNode" instead.

* List(Vec<Node>) becomes List(Vec<SmartNode>) instead
* Env's hashmat has values that are SmartNodes instead of immutable nodes

Make tool support command line args.

Implement dynamic environments like in Lisp In Small Pieces 2.5.1

* A good way to implement exceptions?

eliminate nil

Build a language server for formatting the code
https://code.visualstudio.com/docs/extensionAPI/language-support

Fix pub mods to only expose what's needed.

Use the comparison.html doc to create the rest of the primitives and special forms
