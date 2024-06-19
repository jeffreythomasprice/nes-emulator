```
npm run test:watch
```


TODO precommit, tests, build, lint


TODO proper ast system

ASTNode
- toDebugAST
- toWATAST

writing a WAT AST to string:
- sequence of nodes
- node types
	- comment
	- open, close paren
	- string
- series of strings in between parens can all be on one line
- otherwise every element on it's own line
- except for first string after the open paren, can be on same line
- comments have to be on their own lines
