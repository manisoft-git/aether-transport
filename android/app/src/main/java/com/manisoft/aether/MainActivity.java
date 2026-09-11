package com.manisoft.aether;

import android.app.Activity;
import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.content.IntentFilter;
import android.net.VpnService;
import android.os.Build;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.widget.Button;
import android.widget.TextView;
import android.widget.Toast;

public class MainActivity extends Activity {

    private static final int VPN_REQUEST_CODE = 0xAE78;

    private Button btnToggleVpn;
    private TextView tvVpnStatus;
    private TextView tvUptimeTelemetry;

    private boolean isConnected = false;
    private long connectionStartTime = 0;
    private Handler timerHandler = new Handler(Looper.getMainLooper());
    private Runnable timerRunnable;

    private final BroadcastReceiver statusReceiver = new BroadcastReceiver() {
        @Override
        public void onReceive(Context context, Intent intent) {
            if (AetherVpnService.ACTION_STATUS.equals(intent.getAction())) {
                boolean active = intent.getBooleanExtra(AetherVpnService.EXTRA_IS_RUNNING, false);
                updateUiState(active);
            }
        }
    };

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        btnToggleVpn = findViewById(R.id.btn_toggle_vpn);
        tvVpnStatus = findViewById(R.id.tv_vpn_status);
        tvUptimeTelemetry = findViewById(R.id.tv_uptime_telemetry);

        btnToggleVpn.setOnClickListener(v -> {
            if (isConnected) {
                stopVpnService();
            } else {
                prepareAndStartVpn();
            }
        });

        timerRunnable = new Runnable() {
            @Override
            public void run() {
                if (isConnected && connectionStartTime > 0) {
                    long elapsedSec = (System.currentTimeMillis() - connectionStartTime) / 1000;
                    long hrs = elapsedSec / 3600;
                    long mins = (elapsedSec % 3600) / 60;
                    long secs = elapsedSec % 60;
                    String timeStr = String.format("%02d:%02d:%02d", hrs, mins, secs);
                    tvUptimeTelemetry.setText("Uptime: " + timeStr + " • Zero-RTT Loss Recovery: 100% Active");
                    timerHandler.postDelayed(this, 1000);
                }
            }
        };
    }

    @Override
    protected void onResume() {
        super.onResume();
        IntentFilter filter = new IntentFilter(AetherVpnService.ACTION_STATUS);
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            registerReceiver(statusReceiver, filter, Context.RECEIVER_NOT_EXPORTED);
        } else {
            registerReceiver(statusReceiver, filter);
        }
        updateUiState(AetherVpnService.isRunning);
    }

    @Override
    protected void onPause() {
        super.onPause();
        try {
            unregisterReceiver(statusReceiver);
        } catch (Exception ignored) {}
    }

    private void prepareAndStartVpn() {
        Intent intent = VpnService.prepare(this);
        if (intent != null) {
            // Prompts Android's native system dialog: "App wants to set up a VPN connection. Allow?"
            startActivityForResult(intent, VPN_REQUEST_CODE);
        } else {
            // Already allowed by user
            onActivityResult(VPN_REQUEST_CODE, RESULT_OK, null);
        }
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        if (requestCode == VPN_REQUEST_CODE) {
            if (resultCode == RESULT_OK) {
                startVpnService();
            } else {
                Toast.makeText(this, "VPN permission required for 1-click acceleration", Toast.LENGTH_SHORT).show();
            }
        }
    }

    private void startVpnService() {
        Intent intent = new Intent(this, AetherVpnService.class);
        intent.setAction(AetherVpnService.ACTION_CONNECT);
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            startForegroundService(intent);
        } else {
            startService(intent);
        }
        updateUiState(true);
    }

    private void stopVpnService() {
        Intent intent = new Intent(this, AetherVpnService.class);
        intent.setAction(AetherVpnService.ACTION_DISCONNECT);
        startService(intent);
        updateUiState(false);
    }

    private void updateUiState(boolean active) {
        isConnected = active;
        if (active) {
            btnToggleVpn.setBackgroundResource(R.drawable.btn_circle_active);
            btnToggleVpn.setText(R.string.btn_active);
            tvVpnStatus.setText(R.string.status_connected);
            tvVpnStatus.setTextColor(getResources().getColor(R.color.emerald_glow));
            if (connectionStartTime == 0) {
                connectionStartTime = System.currentTimeMillis();
            }
            timerHandler.removeCallbacks(timerRunnable);
            timerHandler.post(timerRunnable);
        } else {
            btnToggleVpn.setBackgroundResource(R.drawable.btn_circle_idle);
            btnToggleVpn.setText(R.string.btn_activate);
            tvVpnStatus.setText(R.string.status_standby);
            tvVpnStatus.setTextColor(getResources().getColor(R.color.cyan_glow));
            connectionStartTime = 0;
            timerHandler.removeCallbacks(timerRunnable);
            tvUptimeTelemetry.setText("Uptime: 00:00:00 • Standby");
        }
    }
}
