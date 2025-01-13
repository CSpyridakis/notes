# Processes


| Command          | Description                                                                                                    | Usage Example                                    |
|-------------------|----------------------------------------------------------------------------------------------------------------|-------------------------------------------------|
| **`nohup`**      | Prevents a process from being terminated after the user logs out or the session ends.                         | `nohup command &`                               |
| **`&`**          | Runs a process in the background of the current session.                                                     | `command &`                                     |
| **`jobs`**       | Displays a list of current jobs in the shell, including background processes started with `&`.                | `jobs`                                          |
| **`fg`**         | Brings a background job to the foreground.                                                                   | `fg %1` (where `%1` is the job ID)             |
| **`bg`**         | Resumes a suspended job in the background.                                                                   | `bg %2` (where `%2` is the job ID)             |
| **`ps`**         | Lists all running processes, including background and system processes.                                       | `ps aux`                                       |
| **`kill`**       | Sends a signal (e.g., `SIGTERM` or `SIGKILL`) to terminate a process by its PID.                              | `kill 1234` (where `1234` is the PID)          |
| **`disown`**     | Removes a background process from the shell’s job table, making it independent of the session.               | `disown -h %1`                                 |
| **`wait`**       | Waits for one or more background processes to complete before proceeding.                                     | `wait %1`                                      |
| **`timeout`**    | Runs a command for a specified duration and terminates it if the time limit is exceeded.                     | `timeout 30s command`                          |
| **`sleep`**      | Pauses the execution of a script or command for a specified duration.                                         | `sleep 10`                                     |
| **`top`**        | Displays a real-time view of running processes and their resource usage, allowing interaction (e.g., killing).| `top`                                          |
| **`htop`**       | An interactive, user-friendly alternative to `top` for monitoring and managing processes.                     | `htop`                                         |
| **`pkill`**      | Sends a signal (default `SIGTERM`) to processes based on name, user, or other criteria.                      | `pkill my_process`                            |
| **`pgrep`**      | Searches for processes by name and returns their process IDs (PIDs).                                          | `pgrep my_process`                            |
| **`killall`**    | Sends a signal (default `SIGTERM`) to all processes with a given name.                                        | `killall my_process`                          |
| **`pstree`**     | Displays a tree of processes, showing the parent-child relationship between them.                             | `pstree`                                       |
| **`lsof`**       | Lists open files and the processes that opened them. Useful for finding which processes are using specific resources (e.g., files, ports). | `lsof -i :80`                                 |
| **`nice`**       | Starts a process with a modified scheduling priority, influencing how much CPU time it receives.              | `nice -n 10 command`                          |
| **`renice`**     | Alters the priority of a running process.                                                                     | `renice -n 5 -p 1234`                          |


---

## `nohup` and `&`
| Feature                     | `nohup`                                      | `&`                            |
|-----------------------------|----------------------------------------------|--------------------------------|
| **Purpose**                 | Prevents the process from being terminated when the session ends (e.g., logging out). | Runs the process in the background of the same session. |
| **Signal Handling**         | Ignores the `SIGHUP` (hangup) signal.         | Does not handle signals explicitly; the process may terminate on session logout. |
| **Output Handling**         | Redirects output to `nohup.out` by default if not specified. | Does not redirect output; inherits from the terminal unless manually redirected. |
| **Use Case**                | Long-running processes that should continue after logout or session disconnect. | Quick background tasks or temporary jobs in the current session. |
| **Typical Syntax**          | `nohup command &`                           | `command &`                   |
