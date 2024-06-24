# Creating your own server/backend

To create your own server, you really only need to recreate the datatypes, that the client uses. Everything else is up to you.
The client expects an endpoint to send all the currently running RPA-processes to.

Here is an description of what the JSON data looks like:
```json
[
    {
        "engine": "string",
        "computer": "string",
        "started": null or "string",
        "instance": "string",
        "name": null or "string",
        "action": null or Action,
        "trigger": null or "string",
        "flowId": null or "string",
        "parentInstance": null or "string"
    }
]
```
```json
// Action:
{
    "name": "string",
    "functionName": "string",
    "index": number,
    "insideErrorHandling": true or false
}
```

*The reason so many can be null is, because there is a change it will fail to retrieve the information. Eaither because of the Engine implementation or just not getting the information from Windows.*

## Endpoints
The most important endpoint is the the one, that recieves the above data. This endpoint can be changed in the INI-file of the client.

The default endpoint is: `http://<hostname>/api/checkin` and the data can't be changed. The endpoint will be called every 5-10 seconds.

Here is a description of all the ones, that comes with the server, for some inspiration:

| Endpoint          | Method | Description |
| ----------------- | ------ | ----------- |
| `/api/checkin`    | POST | Recieves the above list of running processes. |
| `/api/getrpa`     | GET  | Retrieves the above list. |
| `/api/getfailed`  | GET  | Not implemented. |
| `/api/gethistory` | GET  | Retrieves the past running processes. Max 50. |

Alternative you are more than welcome to reverse enginere the server/website yourself. It should be simple enough to do, since there isn't much to it. You can run everything on your own machine to it (in userspace) to do it.
