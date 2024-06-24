# Connecting to the Power Automate API / MS Dynamics
| Priority | Type/File   |
| -------- | ----------- |
| 1.       | $RW_PACONN  |
| 2.       | `flow.conn` |

## New as of v.0.2.1
As of version 0.2.1, thie flow name *can* be retrieved without the use an API connection on the server. 
This is done by reading the logpath of the current running process and will almost always work. In case it doesn't, 
you can still use the API to retrieve it.

## Prioer to v.0.2.1 (read below)
To make the server display names of the Power Autmate flows, you need allow the server to lookup the names in your PA enviorment. 

This is done by either using the env. variable `RW_PACONN` or creating a file next to the server, called `flow.conn` and make it contain 4 values. An example can be seen [here](#server-file)

This has a little setup, before it can be used.


## Entra ID/Azure Application
You need to create an "Application" in Azure with that server needs to use. The application needs the following permissions: `Flow.Read`.
*Althoug the Flow.Read is unsued as a scope, it's still needed.*

The application will use the `/.default` scope of MS Dynamics. If you don't know what that means, just limit the permissions to `Flow.Read` and all is good.

## Client Secret
*Certificates are currently unsupported, but might be supported later*
The application needs a Client Secret to connect to the Power Automate API. You decide how long it should be valid for, but make shure you update it, when it expires. The Client ID is not needed from here.

## Client ID
The REAL Client ID should be the ID of your Azure application (Application (client) ID) and not the Client ID of the secret itself.

## Teanant ID
The Teanant ID can be found in the overview of tab of Entra ID/Azure.

## Organization ID / Link
The Organization link can be found on the enviorment you want to lookup in. An overview of your enviorments can be found [here](https://admin.powerplatform.microsoft.com/environments).

Once you have found the enviorment you want to use, you need to copy the first 2 parts of the org. link.

Example: `org12345678.crm0`. The actual UUID is not needed.

## Envirorment variable
```Batch
SETX RW_PACONN "ClientId=d5658550-c392-43a4-976c-81abc1162f30&ClientSecret=JaKgIhLwvFwvQIpsOQLKNyKTZLdbOgolGGyCukZs&TeanantId=443f3e5f-3a29-4baa-a8b4-9eefcc82e4e1&OrgId=org12345678.crm0"
```

## server file
The file should look something like this (*Not real data*):
```batch
ClientId=d5658550-c392-43a4-976c-81abc1162f30
ClientSecret=JaKgIhLwvFwvQIpsOQLKNyKTZLdbOgolGGyCukZs
TeanantId=443f3e5f-3a29-4baa-a8b4-9eefcc82e4e1
OrgId=org12345678.crm0
```