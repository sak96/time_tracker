const notification = window.__TAURI__.notification;

export function notify(name) {
  notification.sendNotification({title: name});
}
