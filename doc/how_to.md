# How to contribute to retrowin32

In this doc I hope to collect steps and examples of how to extend or improve
retrowin32.

## How to add a new builtin DLL

See commit 38c7f18b017914c87cd10444294184be981b5a0d, which added a stub DLL.

## How to add a new win32 function

Given some missing function like `TrackMouseEvent`, you can run:

```
$ (cd win32/extract && cargo run TrackMouseEvent)
```

and that program will try to generate the proper function definition from the
metadata published by Microsoft. It also works for structs. Be careful about
mistakes; examine the output for errors.

Whenever you add a function or modify a function's parameters, you must run
`cargo minibuild` again to regenerate the related code.

See commit 9f436905e650eb7c4e996fd799eb260f1238356d for an example.
