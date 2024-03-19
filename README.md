# RPA.Watcher
Keep and eye on your RPA Software 🧐

> [!CAUTION]
> This software is en early stages of development and is being developed as a pass time (aka. very slowly). Use at your own risk.

## About
The idea is to have a closer and more central view of what RPA-processes, flows and what not, you and your organization have running on your machines.

Sometimes the central view is just lagging behind or some times just lying (Power Automate). This software is intended to run along side the RPA process and confirm what is running at the moment.

> [!NOTE]
> Currently it's not able to detect precisely in a human readable format, but that is planned for the future. Like translating an ID to a name.

It consist of two components (well, three):
- The client/agent. This is supposed to run on the OS, that performs the RPA-task. This client is Windows exclusive.
- The server. It can run on both Linux and Windows (maybe MacOS) and should be able to be used in [IIS](https://en.wikipedia.org/wiki/Internet_Information_Services). This you can implement youself if you want.
- the website (wwwroot). This is not a required component of the server module, but you might need the folder to exist for it to work. Feel free to implement this site yourself if you want.

## Supported platforms
- `InstanceID` == The unique ID of the running instance.
- `InstanceName` == The human readable name of the running process.

| Platform       | Implemented | InstanceID | InstanceName |
| -------------- | ----------- | ---------- | ------------ |
| [ProcessRobot](https://learn.microsoft.com/en-us/power-automate/desktop-flows/softomotive-migrator) | Yes | Yes | [Implemented (v0.1.4)](ProcessRobotDB.md) |
| [Power Automate Desktop](https://powerautomate.microsoft.com) | Yes | Yes | [Implemented (v0.1.5)](PowerAutomateAPI.md) |
| [UIPath](https://uipath.com) | Not yet |  | |
| More to come | Not yet |  |  |

## Todo list (for now)
- [x] <s>Add child flow support (client/server).</s>
- [x] <s>Add child flow support (website).</s>
- [x] <s>Add database connection for ProcessRobot (server).</s>
- [x] <s>Add api lookup with flow names for Power Automate (server).</s>
- [ ] Add history overview (server/website).
- [ ] Add failed overview (server/website).

## Building from source
> [!NOTE]
> It's recommended you build the client on Microsoft Windows. The server can be build on Linux and Windows.

The easiest way to build it, is to run the [`Build.cmd`](Build.cmd) file. It will package everything in the target\dist folder afterwards.

Otherwise it's just the standard `cargo build --release`.

## Prerequisites for building
- The Rust compiler (rustc and cargo).
- (Client) Windows SDK (If your using MSVC) or MinGW with `rc.exe` (for GCC).

## Internet Information Service (IIS)
To use the server in IIS, you need the [HttpPlatformHandler](https://www.iis.net/downloads/microsoft/httpplatformhandler) installed on the server. This allows you do redirect all traffic of a site and enable https without much configuration.

What you need to do, is to create `Web.config` file in the root of the hosted directory and place the server binary and the `wwwroot` folder in it.

**An example of `Web.config`:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<configuration>
    <system.webServer>
        <handlers>
            <add name="httpPlatformHandler" path="*" verb="*" modules="httpPlatformHandler" resourceType="Unspecified" requireAccess="Script" />
        </handlers>
        <httpPlatform stdoutLogEnabled="true" startupTimeLimit="20" processPath="<PATH_TO_FOLDER>\rpa-watcher-srv.exe">
        </httpPlatform>
    </system.webServer>
</configuration>
```
