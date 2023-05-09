# rust-k

K is a toy non-turing-complete language that is actually just a stack machine

to push a value on a stack (for now just numbers):
```
1 2 3
```
this will push 1, 2, 3 to a stack, meaning curretn top is 3

to operate on stack top, you can just use whatever operator you want that is currently present in a language

for example:
```
+
```
will pop from the stack twice, add those numbers, push their sum to the stack

K has while loops

```
while <expr> do
<body>
end
```

"expr" will be executed every time before executing the "body", after "expr" execution stack top will be popped
and checked if it is zero or not.
