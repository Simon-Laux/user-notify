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

Currently there is no built-in support for timeouts on notifications (see [#4](https://github.com/Simon-Laux/user-notify/issues/4)).

If you this crate is not for you, then you may like <https://github.com/hoodie/notify-rust>, which is an established crate, but has less feature support on macOS.

### System Requirements:
- macOS 10.14 or above
  - Note for developers: on macOS this only works inside an app package with a “Bundle ID”, also you need an Apple developer account to sign it.
- Windows 10 or above
- Linux with a desktop/notification-daemon which supports the `org.freedesktop.Notifications` dbus protocol. (most modern desktop environments do)

## Usage

TODO <!-- TODO -->: for now look in these places:
- [`examples/test.rs`](./examples/test.rs)
- How delta chat desktop tauri uses it: <https://github.com/search?q=repo%3Adeltachat%2Fdeltachat-desktop+user_notify&type=code>

## History of this crate

This library was initially created as replacement for tauri's notification APIs, because they did not
implement all notification features that we needed for our project
of porting the [Delta Chat instant messenger](https://github.com/deltachat/deltachat-desktop) from electron to [tauri](https://tauri.app/)
(Basic features like [reacting to clicks](https://github.com/tauri-apps/plugins-workspace/issues/2150#issuecomment-3782406762) on notifications were missing in the rust api).


### Features by operating system

Legend:
- ✅ — is supported
- `#issue-number` — tracking issue for implementing it
- ❌ — not planned, since platform has no support for it (yet)
- NO — not necessary on platform, so implemented as “no operation”
- 🏃 — does not work across sessions, just in current session


| What                                                                                                                                         | macOS | Linux                                                    | Windows                                                  | API                                                                                                                                                                                                    |
| -------------------------------------------------------------------------------------------------------------------------------------------- | ----- | -------------------------------------------------------- | -------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Primary description                                                                                                                          | ✅     | ✅                                                        | ✅                                                        | [`NotificationBuilder.title`](https://docs.rs/user-notify/latest/user_notify/struct.NotificationBuilder.html#method.title)                                                                             |
| Main content                                                                                                                                 | ✅     | ✅                                                        | ✅                                                        | [`NotificationBuilder.body`](https://docs.rs/user-notify/latest/user_notify/struct.NotificationBuilder.html#method.body)                                                                               |
| secondary description                                                                                                                        | ✅     | ❌                                                        | ✅                                                        | [`NotificationBuilder.subtitle`](https://docs.rs/user-notify/latest/user_notify/struct.NotificationBuilder.html#method.subtitle)                                                                       |
| Image Attachment                                                                                                                             | ✅     | ✅                                                        | ✅                                                        | [`NotificationBuilder.set_image`](https://docs.rs/user-notify/latest/user_notify/struct.NotificationBuilder.html#method.set_image)                                                                     |
| Override app icon                                                                                                                            | ❌     | ✅                                                        | ✅                                                        | [`NotificationBuilder.set_icon`](https://docs.rs/user-notify/latest/user_notify/struct.NotificationBuilder.html#method.set_icon)                                                                       |
| Group notifications <br>by thread                                                                                                            | ✅     | ❌                                                        | [#3](https://github.com/Simon-Laux/user-notify/issues/3) | [`NotificationBuilder.set_thread_id`](https://docs.rs/user-notify/latest/user_notify/struct.NotificationBuilder.html#method.set_thread_id)                                                             |
| Set category/template with actions                                                                                                           | ✅     | ✅                                                        | ✅                                                        | [`NotificationBuilder.set_category_id`](https://docs.rs/user-notify/latest/user_notify/struct.NotificationBuilder.html#method.set_category_id)                                                         |
| Set notification data                                                                                                                        | ✅     | ✅🏃                                                      | ✅                                                        | [`NotificationBuilder.set_user_info`](https://docs.rs/user-notify/latest/user_notify/struct.NotificationBuilder.html#method.set_user_info)                                                             |
| Persistent notifications across sessions<br>(keeping their data even when you restart the app / handle notifications from previous sessions) | ✅     | ❌                                                        | ✅                                                        | -                                                                                                                                                                                                      |
| Get permission state                                                                                                                         | ✅     | ❌                                                        | NO                                                       | [`NotificationManager .get_notification_permission_state`](https://docs.rs/user-notify/latest/user_notify/trait.NotificationManager.html#tymethod.get_notification_permission_state)                   |
| Ask for permission                                                                                                                           | ✅     | ❌                                                        | ❌                                                        | [`NotificationManager .first_time_ask_for_notification_permission`](https://docs.rs/user-notify/latest/user_notify/trait.NotificationManager.html#tymethod.first_time_ask_for_notification_permission) |
| Remove all notifications                                                                                                                     | ✅     | ✅🏃                                                      | ✅                                                        | [`NotificationManager .remove_all_delivered_notifications`](https://docs.rs/user-notify/latest/user_notify/trait.NotificationManager.html#tymethod.remove_all_delivered_notifications)                 |
| Remove notifications by id                                                                                                                   | ✅     | ✅🏃                                                      | ✅                                                        | [`NotificationManager .remove_delivered_notifications`](https://docs.rs/user-notify/latest/user_notify/trait.NotificationManager.html#tymethod.remove_delivered_notifications)                         |
| Get still active notifications                                                                                                               | ✅     | ✅🏃                                                      | ✅🏃                                                      | [`NotificationManager .get_active_notifications`](https://docs.rs/user-notify/latest/user_notify/trait.NotificationManager.html#tymethod.get_active_notifications)                                     |
| Action: Button                                                                                                                               | ✅     | [#1](https://github.com/Simon-Laux/user-notify/issues/1) | [#2](https://github.com/Simon-Laux/user-notify/issues/2) | [`NotificationCategoryAction::Action`](https://docs.rs/user-notify/latest/user_notify/enum.NotificationCategoryAction.html#variant.Action)                                                             |
| Action: reply input field                                                                                                                    | ✅     | [#21](https://github.com/Simon-Laux/user-notify/issues/21)                                                      | [#2](https://github.com/Simon-Laux/user-notify/issues/2) | [`NotificationCategoryAction::TextInputAction`](https://docs.rs/user-notify/latest/user_notify/enum.NotificationCategoryAction.html#variant.TextInputAction)                                           |

Platform specific API:

| What                                                                                                        | macOS | Linux | Windows | API                                                                                                                                                    |
| ----------------------------------------------------------------------------------------------------------- | ----- | ----- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Set App icon to be round                                                                                    | ❌     | ❌     | ✅       | [`NotificationBuilder.set_icon_round_crop`](https://docs.rs/user-notify/latest/user_notify/struct.NotificationBuilder.html#method.set_icon_round_crop) |
| Set [xdg notification Category](https://specifications.freedesktop.org/notification/latest/categories.html) | ❌     | ✅     | ❌       | [`NotificationBuilder.set_xdg_category`](https://docs.rs/user-notify/latest/user_notify/struct.NotificationBuilder.html#method.set_xdg_category)       |
| Override app name                                                                                           | ❌     | ✅     | ❌       | [`NotificationBuilder.set_xdg_app_name`](https://docs.rs/user-notify/latest/user_notify/struct.NotificationBuilder.html#method.set_xdg_app_name)       |


## Testing the example

### Linux

```
RUST_LOG=debug cargo run --example test
```

### macOS
On macOS you need a signed app package, otherwise notifications don't work.
So may only work fully in released/packaged versions when you use tauri, you can use the "mock" implementation in debug mode which just logs to the console.

You can build and package the example (`examples/test.rs`) for macOS with this helper script:
```sh
security find-identity -v -p codesigning
# replace the zeros with the signing key you want to use
APPLE_SIGNING_IDENTITY=00000000000000000000000000000000000000 ./test_example_macos.sh
```

## Windows
TODO: instructions for windows (I don't remember if it also needed custom steps)
<!--- TODO -->

## Useful Links

These links can be useful for reference when contributing to this plugin.

- macOS:
  - https://developer.apple.com/documentation/usernotifications
  - https://lib.rs/crates/objc2-user-notifications/features#feature-UNNotificationCategory
  - overview of objc2 crate: [youtube: 13th Bevy Meetup - Mads - Bevy on iOS](https://www.youtube.com/watch?v=wv76dB2yATk)
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
