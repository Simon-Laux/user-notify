## User Notifications

Simple library to implement all notification features that we need for delta tauri.

Notifications stay until they are closed by the user or by the app.

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

## Future

For the future we could consider to implement https://developer.apple.com/documentation/usernotifications/implementing-communication-notifications to show a user avatar in the notifications.
