# Rust Modules Made Easy
Modules are parts of a crate outside of main.rs/lib.rs file. It's how crates are organize the codebase to make it easier to work with. There are
three ways that we can split things into modules:

1. All files are in the src directory.
2. Directories within the src directory that have a .rs file of the same name in the src directory.
3. Directories within the src directory that have a mod.rs file within them.

## All Files in src Directory
This is the most straight forward way of doing this, and is the preferred way if your crate/project is very small. It would look something like:  
  
project_dir  
|  
--- src  
|&nbsp;&nbsp; |  
|&nbsp;&nbsp;&nbsp;-- main.rs  
|&nbsp;&nbsp;&nbsp;-- utils.rs  
  
With this method the only place you need to declare your module (i.e.; mod module_name;) is at the top of main.rs (or lib.rs if it's a library).  

## Directories with their .rs Files
This is the modern Rust way of doing things for crates/projects that are larger and likely to get unwieldy doing it with all of the source files in the src directory.
It looks something like the following:  
  
project_dir  
|  
--- src  
|&nbsp;&nbsp; |  
|&nbsp;&nbsp;&nbsp;-- main.rs  
|&nbsp;&nbsp;&nbsp;-- utils.rs  
|&nbsp;&nbsp;&nbsp;-- utils  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|    
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- math.rs  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- bounds.rs  
|&nbsp;&nbsp;&nbsp;-- shapes.rs  
|&nbsp;&nbsp;&nbsp;-- shapes  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- circle.rs  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- rectangle.rs  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- triangle.rs  
  
Unlike the previous method, you have to add `mod module_name;` to the top of main.rs or lib.rs, but within the module_name.rs file you also have to add `mod file_name;` at the top
for every file within the directory. This way all of the modules are declared for the project and the compiler can see them.  

## Directories with mod.rs Files
This is the older way of doing things, but it still works fine and sometimes can actually be cleaner looking. The down side is that there are multiple files named `mod.rs` in
your project and can be confusing which one you're on if you have a lot of them. This method structure looks similar to the following:  
  
project_dir  
|  
--- src  
|&nbsp;&nbsp; |  
|&nbsp;&nbsp;&nbsp;-- main.rs  
|&nbsp;&nbsp;&nbsp;-- utils  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- mod.rs    
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- math.rs  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- bounds.rs  
|&nbsp;&nbsp;&nbsp;-- shapes  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- mod.rs  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- circle.rs  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- rectangle.rs  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- triangle.rs  
  
Just like the previous method, this method requires you to do `mod module_name;` at the top of main.rs/lib.rs as well as `mod file_name;` at the top of each mod.rs file for every file you
create in the directory.  
  
---  
  
## Why Use Modules?
No matter which way you choose to declare modules for your project, it's better to have similar functionality chunked into smaller modules rather than everything in one single file, especially
not having everything in main.rs.