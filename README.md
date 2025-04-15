usage:
- Compile to IR code
	`cargo run c path/to/file.bdg (optional)path/to/output/file.xyz`
- Run code
	`cargo run i path/to/file.bdg`

- Delimitators:
`() {} [] : ; . , + - * / % > = < " ' ? @`
- For comment use `#`
- Operators:
`+ - * / % > < == >= <= & | ! != ? @`
- For reference we use `@`. It's basically a pointer. usage: ```
```
num x = 0;
do_smth(@x);
fxn do_smth(num @ptr) { ptr = ptr + 1; }
```
- For checking if a reference is valid or not, we use `?` before the reference, if it is valid, it returns `true` else it returns `false`. usage : ```
``` 
if (?ptr[0]) {
	ptr[0] = ptr[0] + 1 #auto deref
}
```
- Keywords:
`true`, `false` , `if` , `else` , `fxn` , `while` , `break` , `continue` , `import`, `return` , `num` , `str` , `bool` , `null` , `export`

- Strings and escaped characters in the are resolved at compile time itself
- Numbers are 64 bit floating point numbers and are also resolved at compile time.
