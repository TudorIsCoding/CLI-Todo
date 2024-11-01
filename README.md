## CLI-Todo
A simple and efficient command-line to-do list manager written in Rust Language. I used the crate rusqilte for buildinga database, manage data stora. The commands are using SQLite for searchin in the database.
## Instalation
Install and setup cargo
# Linux/Windows/Mac 
```bash 
cargo install cli_todo_list_sqlite
```


```bash
## Usage 
# Add a new task
todo add <task_name>

# View all tasks
todo list 

# Remove a task
todo remove <task_id> 

# Mark task as done 
todo done <task_id>

# Remove all the task from the database
todo reset 

# Sorting tasks ascending or descending by the name 
todo sort -asc/-desc

# Listing the remaining tasks 
todo rmn
```


