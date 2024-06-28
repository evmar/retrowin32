This directory contains generated Windows system DLLs.

These DLLs don't do anything, they just immediately invoke sysenter which will
transfer control into Rust code within retrowin32. We generate real DLL files
though so the system DLLs behave similarly at runtime to real DLLs.

These are generated from the dllexport annotations the functions under winapi.
The win32/derive generator parses them and generated the assembly/.def inputs.
