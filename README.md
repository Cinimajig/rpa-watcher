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
- `ClickableLink` == Contains a clickable link on the website.
- `AzureData` == Contains data related to Microsoft Azure.

| Platform       | Implemented | InstanceID | InstanceName | ClickableLink | AzureData |
| -------------- | ----------- | ---------- | ------------ | ------------- | --------- |
| [ProcessRobot](https://learn.microsoft.com/en-us/power-automate/desktop-flows/softomotive-migrator) | Yes | Yes | Planned | No | No |
| [Power Automate Desktop](https://powerautomate.microsoft.com) | Yes | Yes | Planned | Yes | Yes |
| [UIPath](https://uipath.com) | Not yet |  |  |  |  |
| More to come | Not yet |  |  |  |  |

## Todo list (for now)
- [x] Add child flow support (client/server).
- [x] Add child flow support (website).
- [ ] Add database connection for ProcessRobot (server. Low priority).
- [ ] Add api lookup with flow names for Power Automate (server).
- [ ] Add failed overview (server/website).

## Building from source
> [!NOTE]
> It's recommended you build the client on Microsoft Windows. The server can be build on Linux and Windows.

The easiest way to build it, is to run the [`Build.cmd`](Build.cmd) file. It will package everything in the target\dist folder afterwards.

Otherwise it's just the standard `cargo build --release`.

## Prerequisites for building
- The Rust compiler (rustc and cargo).
- (Client) Windows SDK (If your using MSVC) or MinGW with `rc.exe` (for GCC).
