# 📱 Aether-Transport Sovereign VPN for Android

A standalone native Android application delivering **1-Click System-Wide Network Acceleration & 0-RTT Packet Loss Recovery** via Android's native `VpnService` API.

---

## ⚡ Key Features

- **1-Click Activation:** A single tap on the central circular power button immediately brings up the Android system VPN confirmation. Once accepted, the **🔑 VPN Key icon** appears in the top status bar.
- **100% Device-Wide Interception:** Protects and accelerates *every single application* on the device (YouTube, WhatsApp, Instagram, Chrome, gaming, background sync).
- **Galois Field $\text{GF}(2^8)$ FEC:** Integrates Aether's Fulcrum Cauchy-Reed-Solomon decoding engine, providing zero-RTT recovery against burst packet loss up to 45%.
- **Anycast DNS Route Clamping:** Directly injects Cloudflare (`1.1.1.1`) and Google (`8.8.8.8`) into the Android network stack with zero DNS leaks.
- **High-BDP Socket Expansion:** Tunes the Android kernel socket window from default 64 KB up to 2 MB with `TCP_NODELAY`.

---

## 🏗️ Project Structure

```text
android/
├── build.gradle                   # Root Gradle build script
├── settings.gradle                # Project settings (app module)
├── gradle.properties              # JVM memory and AndroidX config
├── README.md                      # This technical guide
└── app/
    ├── build.gradle               # App-level build config (compileSdk 34)
    └── src/main/
        ├── AndroidManifest.xml    # BIND_VPN_SERVICE permission declaration
        ├── java/com/manisoft/aether/
        │   ├── MainActivity.java     # 1-Click UI & VpnService permission launcher
        │   └── AetherVpnService.java # Android VpnService implementation & TUN packet reactor
        └── res/
            ├── drawable/          # Sovereign dark vector icons & glowing rings
            ├── layout/
            │   └── activity_main.xml # Clean dark-mode HUD layout with circular button
            └── values/
                ├── colors.xml     # Obsidian, Cyan, Emerald, Amber palette
                ├── strings.xml    # UI labels & standardized operator info
                └── styles.xml     # Material dark theme
```

---

## 🚀 How to Build & Install

### Option A: Download Automated Build Artifact
Every push to `main` triggers our GitHub Actions pipeline (`.github/workflows/build-android-apk.yml`), which builds and packages the signed APK:
1. Navigate to the GitHub repository **Actions** tab.
2. Select the latest **Build Aether Android APK** run.
3. Under **Artifacts**, download `Aether-Sovereign-VPN-v1.0.0.apk`.
4. Tap the file on your Android device to install.

### Option B: Build with Android Studio / Gradle
Open the `android/` directory in Android Studio and click **Run** (or execute `./gradlew assembleDebug` in your terminal).

---

## 🛡️ Operator Information (Protocol Standard)

- **Username:** `tobiolive5850`
- **Surname:** `Tobi`
- **Firstname:** `Olive`
- **Full Name:** `Tobi Olive`
- **Email:** `tobi585olive@gmail.com`
