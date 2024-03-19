# Connecting to a Control Desk (Aka. Database)
| Encoding | File      |
| -------- | --------- |
| UTF-8    | `db.conn` |

To make the server display names of the Robot running and the trigger, you need to let the server connect to the Database Control Desk uses.

This is done by creating a file next to the server, called `db.conn` and make it contain a connection string to the server. You can create a user for this purpose, that has read access to the table: `job_instances_running`. The connection string should be in a single line.

The file will be read, when the server launches and is then cached. Next time the server starts or restarts it will read the file again.