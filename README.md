# Wal

Wal is lisp-inspired scripting language with really weird tokens written in rust.

## Docs

### Instalation

```
$ git clone https://github.com/TENMAJKL/wal
$ cargo run <file.wal>
```

### Introduction

Wal is not directly functional programming language, but everything you do its via functions.

Each function is inside `[]`, first word is function name and other words (separated by space) are arguments.

Lets see a quick example:

```wal
[-> 'Hello world!']
```

On this example we can see calling function `->` which prints all given arguments. The first argument (`'Hello world!'`) is string literal, so output will be `Hello world!`.

### Literals

Wal currently supports only string literarls and positive integers.

For comments is used `#` which ends with new line:

```wal
; demonstration of comments

[-> 'foo'] # this is print 

; [-> 'bar'] this wont evaluate
```

### Operators

Everything except literarls in wal is a function even operators, so for adding 2 numbers we use function `+`:

```wal
[+ 1 2]
```

Function `+` returns sum of given arguments, so if we want to print it:

```wal
[-> [+ 1 2]]
```

The same goes for `-`:

```wal
[-> [- 1 2]]
```

For comparing, there is function `==`:

```wal
[-> [== 1 2]] # false
[-> [== 1 1]] # true
[-> [== 'foo' 'foo']] # true
```

And its strict:

```wal
[-> [== '1' 1]] # false
```

We can use this operator for

### If statements

Wal has if function `=<` called "sad face operator" which works like ternary operator:

```wal
[=< [== 1 2]
    [-> 'foo']
    [-> 'bar']
]
```

If first argument is true, it evaluates and returns output of second argument, if not the same goes for third.

```wal
[-> 
    [=< [== 1 2]
        'foo'
        'bar'
    ]
]
```

Why `=<`? Because this operator literally looks like branch.

### Variables

Wal has simple system of variables, all the manipulation is done with function `$`:

```wal
[$ foo 10] # created variable called foo with value 10
[-> [$ foo]] # accessing variable 
```

If the function has only 1 argument it returns value of given variable. If it has 2 arguments it sets the value to the variable and returns the value.

### Arrays

Function `@` creates array and returns it. Its not saved:

```
[@ 1 2 3 'foo']
```

So if we want to save it we have to put it in variable explicitely

```wal
[$ array
    [@ 1 2 3 'foo']
]
```

Arrays can be printed:

```wal
[-> [@ 1 2 'foo']] # Array: 1 2 'foo'
```

#### Pushing to array

Function `@>` pushes all arguments to the top of array given as first argument.

```wal

[-> [@> [@ 1 2] 3]] # 1 2 3

More complex example:

```wal

[$ array
    [@ 1 2 3 'foo']
]

[$ array
    [@>
        [$ array]
        1
    ] 
]

[-> [$ array]] # 1 2 3 'foo' 1

```

#### Indexing

For indexing there is `@$` function which returns element on index from first argument or if there is third argument it sets item on given index to the third argument value and returns the array:

```wal

[$ array
    [@ 1 2 3]
]

[->
    [@$ 
        [$ array]
        1
    ]
] ; outputs 2

[-> [@$ [$ array] 1 3]] # outputs Array: 1 3 3

```

### Retreving user input

Wal has function `<-` which returns value from standart input:

```wal

[$ name [<-]]
[-> 'Hello ' [$ name]]
```
