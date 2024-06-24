# Connecting to a Control Desk (Aka. Database)
| Priority | Type/File   |
| -------- | ----------- |
| 1.       | $RW_DBCONN  |
| 2.       | `db.conn`   |

To make the server display names of the Robot running and the trigger, you need to let the server connect to the Database Control Desk uses.

This is done by either using the env. variable `RW_DBCONN` or creating a file next to the server, called `db.conn` and make it contain a connection string to the server. You can create a user for this purpose, that has read access to the table: `job_instances_running`. The connection string should be in a single line.

The file will be read, when the server launches. Next time the server starts up, it will read the file again.
