# `user-notify`

Simple library to implement user facing notifications in end-user applications on macOS, Linux and Windows.

The name `user-notify` is inspired by how the system API is called in the Apple ecosystem (“User Notifications”).

## What can it do?

You can send notifications to your users using this crate.
Goal of this crate is to provide an API that just works
and offers enough of the platform specific API to be useful for creating full apps like instant messengers.

- Uses system APIs directly, you can contribute to this crate to add options for using the rest of the system API.
  - In the future this crate could even support contact avatars and macOS focus.
- Buttons in notifications (macOS, todo: windows, Linux).
- Inline Reply to notifications (macOS, todo: windows).
- Images in notifications.
- Notifications are identified by metadata that you can add to them (windows, macOS).
- Notification can persist across sessions and can be used to start your app (windows, macOS).
- Async API with [Tokio](https://tokio.rs/).
- Not many dependencies.

If you this crate is not for you, then you may like <https://github.com/hoodie/notify-rust>, which is an established crate, but has less feature support on macOS.

### System Requirements:
- macOS 10.14 or above
  - Note for developers: on macOS this only works inside an app package with a “Bundle ID”, also you need an Apple developer account to sign it.
- Windows 10 or above
- Linux with a desktop/notification-daemon which supports the `org.freedesktop.Notifications` dbus protocol. (most modern desktop environments do)

## Usage

TODO <!-- TODO -->

## History of this crate

This library was initially created as replacement for tauri's notification APIs, because they did not
implement all notification features that we needed for our project
of porting the [Delta Chat instant messenger](https://github.com/deltachat/deltachat-desktop) from electron to [tauri](https://tauri.app/)
(Basic features like reacting to clicks on notifications were missing in the rust api).


### Features by operating system

Legend:
- ✅ — is supported
- `#issue-number` — tracking issue for implementing it
- ❌ — not planned, since platform has no support for it (yet)
- NO — not necessary on platform, so implemented as “no operation”
- 🏃 — does not work across sessions, just in current session

| what                                                                                                                                         | api                                                               | MacOS | Linux                                                    | Windows                                                  |
| -------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- | ----- | -------------------------------------------------------- | -------------------------------------------------------- |
| Primary description                                                                                                                          | `Notification.title`                                              | ✅     | ✅                                                        | ✅                                                        |
| Main content                                                                                                                                 | `Notification.body`                                               | ✅     | ✅                                                        | ✅                                                        |
| secondary description                                                                                                                        | `Notification.subtitle`                                           | ✅     | ❌                                                        | ✅                                                        |
| Image Attachment                                                                                                                             | `Notification.set_image`                                          | ✅     | ✅                                                        | ✅                                                        |
| Override app icon                                                                                                                            | `Notification.set_icon`                                           | ❌     | ✅                                                        | ✅                                                        |
| Group notifications <br>by thread                                                                                                            | `Notification.set_thread_id`                                      | ✅     | ❌                                                        | [#3](https://github.com/Simon-Laux/user-notify/issues/3) |
| Set category/template with actions                                                                                                           | `Notification.set_category_id`                                    | ✅     | ✅                                                        | ✅                                                        |
| Set notification data                                                                                                                        | `Notification.set_user_info`                                      | ✅     | ✅🏃                                                      | ✅                                                        |
| Persistent notifications across sessions<br>(keeping their data even when you restart the app / handle notifications from previous sessions) | -                                                                 | ✅     | ❌                                                        | ✅                                                        |
| Get permission state                                                                                                                         | `NotificationManager .get_notification_permission_state`          | ✅     | ❌                                                        | NO                                                       |
| Ask for permission                                                                                                                           | `NotificationManager .first_time_ask_for_notification_permission` | ✅     | ❌                                                        | ❌                                                        |
| Remove all notifications                                                                                                                     | `NotificationManager .remove_all_delivered_notifications`         | ✅     | ✅🏃                                                      | ✅                                                        |
| Remove notifications by id                                                                                                                   | `NotificationManager .remove_delivered_notifications`             | ✅     | ✅🏃                                                      | ✅                                                        |
| Get still active notifications                                                                                                               | `NotificationManager .get_active_notifications`                   | ✅     | ✅🏃                                                      | ✅🏃                                                      |
| Action: Button                                                                                                                               | `NotificationCategoryAction::Action`                              | ✅     | [#1](https://github.com/Simon-Laux/user-notify/issues/1) | [#2](https://github.com/Simon-Laux/user-notify/issues/2) |
| Action: reply input field                                                                                                                    | `NotificationCategoryAction::TextInputAction`                     | ✅     | ❌                                                        | [#2](https://github.com/Simon-Laux/user-notify/issues/2) |

Platform specific API:

| what                                                                                                        | api                                | MacOS | Linux | Windows |
| ----------------------------------------------------------------------------------------------------------- | ---------------------------------- | ----- | ----- | ------- |
| Set App icon to be round                                                                                    | `Notification.set_icon_round_crop` | ❌     | ❌     | ✅       |
| Set [xdg notification Category](https://specifications.freedesktop.org/notification/latest/categories.html) | `Notification.set_xdg_category`    | ❌     | ✅     | ❌       |
| Override app name                                                                                           | `Notification.set_xdg_app_name`    | ❌     | ✅     | ❌       |


## Testing the example

### Linux

```
RUST_LOG=debug cargo run --example test
```

### MacOS
On MacOS you need a signed app package, otherwise notifications don't work.
<!-- TODO confirm this: -->So may only work fully in released/packaged versions when you use tauri?

You can build and package the example (`examples/test.rs`) for macOS with this helper script:
```
./test_example_macos.sh
```

## Windows
TODO: instructions for windows (I don't remember if it also needed custom steps)
<!--- TODO -->

## Useful Links

These links can be useful for reference when contributing to this plugin.

- macOS:
  - https://developer.apple.com/documentation/usernotifications
  - https://lib.rs/crates/objc2-user-notifications/features#feature-UNNotificationCategory
- Windows:
  - https://docs.rs/tauri-winrt-notification/latest/tauri_winrt_notification/struct.Toast.html
  - https://learn.microsoft.com/en-us/uwp/api/windows.ui.notifications.toastnotification
- xdg / Linux:
  - https://specifications.freedesktop.org/notification-spec/latest/protocol.html
  - https://github.com/hoodie/notify-rust

## Contributing

Contributions are welcome.
Just be nice to everyone, if you are unsure what being nice and fair means, then refer to <https://delta.chat/en/community-standards>.

## Credits

### Contributors

- [Simon Laux](https://github.com/Simon-Laux)
- [Wofwca](https://github.com/WofWca)
- ...maybe you too?

### Funding

- Since this was part of the Delta Chat Tauri project, the initial work on this was funded through [NGI0 Entrust](https://nlnet.nl/entrust), a fund established by [NLnet](https://nlnet.nl) with financial support from the European Commission's [Next Generation Internet](https://ngi.eu) program. Learn more at the [NLnet project page](https://nlnet.nl/project/DeltaTauri).
- Contact us if you want to sponsor a feature: git@simonlaux.de
