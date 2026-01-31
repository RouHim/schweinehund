# Android Push Notifications Setup

This guide explains how to receive push notifications from Schweinehund on your Android phone.

## Overview

Schweinehund uses **ntfy.sh**, a simple HTTP-based push notification service. Notifications are sent to the "schweinehund" topic on the local server without any account creation or authentication needed.

## Prerequisites

- Android phone (6.0 or higher)
- WiFi or network connection to your local network where Schweinehund is running
- Phone able to reach `http://schweinehund.local:8090` or the IP address of your Schweinehund server

## Step-by-Step Setup

### 1. Install the ntfy Android App

1. Open **Google Play Store** on your Android phone
2. Search for **"ntfy"** (published by binwiederhier)
3. Tap **Install**
4. Wait for installation to complete

**Alternative:** Download from [F-Droid](https://f-droid.org/packages/io.heckel.ntfy/) if you prefer an open-source app store

### 2. Add Schweinehund Subscription

1. Open the **ntfy app**
2. Tap the **"+"** button or **"Add subscription"**
3. Enter the server URL and topic:
   - **Server:** `http://schweinehund.local:8090` (or your server's IP, e.g., `http://192.168.1.100:8090`)
   - **Topic:** `schweinehund`
4. Tap **Subscribe**

You should now see the "schweinehund" topic in your subscription list.

### 3. Configure Notification Settings (Optional)

In the ntfy app, you can customize:

- **Notification sound:** Choose or mute
- **Notification priority:** Set to High for urgent tasks
- **Do Not Disturb:** Schedule quiet hours
- **Vibration:** Enable/disable

## Testing the Setup

Once you've subscribed, test if notifications work:

### From Your Computer

Open a terminal and send a test notification:

```bash
curl -d "Test notification from Schweinehund!" http://localhost:8090/schweinehund
```

Or if on a different machine (replace IP):

```bash
curl -d "Test notification from Schweinehund!" http://192.168.1.100:8090/schweinehund
```

You should receive a notification on your Android phone within a few seconds.

### From Schweinehund Web App

In the Schweinehund settings, there's a "Send Test Notification" button that will test your setup.

## Troubleshooting

### Notifications Not Arriving

1. **Check network connectivity:**
   - On your phone, open a browser
   - Navigate to `http://schweinehund.local:8090` (or your IP)
   - You should see the ntfy web interface

2. **Check subscription:**
   - In ntfy app, tap the "schweinehund" topic
   - Look for a test message you sent from your computer

3. **Check Docker container:**
   ```bash
   docker compose ps ntfy
   # Should show "Up (healthy)"
   ```

4. **Check logs:**
   ```bash
   docker compose logs ntfy | tail -20
   ```

### "Cannot reach server" Error

- Verify you're on the same WiFi network as the Schweinehund server
- Check if the IP address or hostname is correct
- Ping the server: `ping schweinehund.local` (or `ping 192.168.1.100`)
- Check firewall: Port 8090 must be accessible

### Notifications Appear But Immediately Disappear

- This is normal for old messages or test messages
- Check the "History" tab in ntfy app to see all messages

## When You'll Get Notifications

Schweinehund sends notifications for:

- **Daily reminder (09:00):** "Time to do your tasks!"
- **Zone reminder:** "Today is [zone name]"
- **Weekend special task:** Reminder for large tasks on weekends
- **Weekly reset (Monday 00:00):** "New week! Tasks reset"

All times are based on your server's local time (Europe/Berlin).

## Managing Subscriptions

### Change Settings

1. In ntfy app, tap and hold the "schweinehund" topic
2. Select **Edit** or **Settings**
3. Modify notification preferences
4. Tap **Save**

### Remove Subscription

1. In ntfy app, tap and hold the "schweinehund" topic
2. Select **Unsubscribe** or **Delete**

## Advanced: Using a Different Server URL

If your server doesn't respond to `schweinehund.local`:

1. Find your server's IP address
   - On Linux/Mac: `hostname -I` or `ipconfig getifaddr en0`
   - On TrueNAS: Settings → Network → IPv4 Address

2. Use that IP in the ntfy app: `http://192.168.X.X:8090/schweinehund`

## Security Note

Since this is a local network setup:
- No authentication is required
- Notifications are sent over HTTP (not HTTPS) for simplicity
- Only accessible from your local network
- No data is sent to external servers

If you want to secure the connection, set up HTTPS with self-signed certificates (Caddy/nginx reverse proxy).

## Support

For issues with the ntfy app itself, see the [ntfy documentation](https://docs.ntfy.sh/).

For Schweinehund-specific issues, check the main application logs.
