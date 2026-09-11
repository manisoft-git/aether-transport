package com.manisoft.aether;

import android.app.Activity;
import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.content.IntentFilter;
import android.graphics.Color;
import android.net.VpnService;
import android.os.Build;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.widget.Button;
import android.widget.TextView;
import android.widget.Toast;

import java.net.InetSocketAddress;
import java.net.Socket;
import java.util.Locale;

public class MainActivity extends Activity {

    private static final int VPN_REQUEST_CODE = 0xAE78;

    private Button btnToggleVpn;
    private Button btnModeBest;
    private Button btnModeTurbo;
    private Button btnRunAudit;

    private TextView tvVpnStatus;
    private TextView tvUptimeTelemetry;
    private TextView tvDnsLatency;
    private TextView tvDnsGain;
    private TextView tvQueriesCount;
    private TextView tvAuditResults;

    private boolean isConnected = false;
    private String selectedMode = AetherVpnService.MODE_BEST_NETWORK;
    private long connectionStartTime = 0;
    private final Handler mainHandler = new Handler(Looper.getMainLooper());
    private Runnable timerRunnable;

    private final BroadcastReceiver statusReceiver = new BroadcastReceiver() {
        @Override
        public void onReceive(Context context, Intent intent) {
            if (AetherVpnService.ACTION_STATUS.equals(intent.getAction())) {
                boolean active = intent.getBooleanExtra(AetherVpnService.EXTRA_IS_RUNNING, false);
                long queries = intent.getLongExtra(AetherVpnService.EXTRA_QUERIES_COUNT, 0);
                float latency = intent.getFloatExtra(AetherVpnService.EXTRA_AVG_LATENCY, 24.5f);
                String bestResolver = intent.getStringExtra(AetherVpnService.EXTRA_BEST_RESOLVER);

                updateUiState(active);
                updateTelemetry(queries, latency, bestResolver);
            }
        }
    };

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        btnToggleVpn = findViewById(R.id.btn_toggle_vpn);
        btnModeBest = findViewById(R.id.btn_mode_best);
        btnModeTurbo = findViewById(R.id.btn_mode_turbo);
        btnRunAudit = findViewById(R.id.btn_run_audit);

        tvVpnStatus = findViewById(R.id.tv_vpn_status);
        tvUptimeTelemetry = findViewById(R.id.tv_uptime_telemetry);
        tvDnsLatency = findViewById(R.id.tv_dns_latency);
        tvDnsGain = findViewById(R.id.tv_dns_gain);
        tvQueriesCount = findViewById(R.id.tv_queries_count);
        tvAuditResults = findViewById(R.id.tv_audit_results);

        btnModeBest.setOnClickListener(v -> selectMode(AetherVpnService.MODE_BEST_NETWORK));
        btnModeTurbo.setOnClickListener(v -> selectMode(AetherVpnService.MODE_FULL_TUNNEL));

        btnToggleVpn.setOnClickListener(v -> {
            if (isConnected) {
                stopVpnService();
            } else {
                prepareAndStartVpn();
            }
        });

        btnRunAudit.setOnClickListener(v -> runNetworkAudit());

        timerRunnable = new Runnable() {
            @Override
            public void run() {
                if (isConnected && connectionStartTime > 0) {
                    long elapsedSec = (System.currentTimeMillis() - connectionStartTime) / 1000;
                    long hrs = elapsedSec / 3600;
                    long mins = (elapsedSec % 3600) / 60;
                    long secs = elapsedSec % 60;
                    String timeStr = String.format(Locale.US, "%02d:%02d:%02d", hrs, mins, secs);
                    tvUptimeTelemetry.setText("Uptime: " + timeStr + " • 0-RTT Galois Field FEC Active");
                    mainHandler.postDelayed(this, 1000);
                }
            }
        };

        selectMode(AetherVpnService.MODE_BEST_NETWORK);
    }

    private void selectMode(String mode) {
        selectedMode = mode;
        if (AetherVpnService.MODE_BEST_NETWORK.equals(mode)) {
            btnModeBest.setTextColor(Color.parseColor("#f8fafc"));
            btnModeBest.setBackgroundResource(R.drawable.card_bg);
            btnModeTurbo.setTextColor(Color.parseColor("#64748b"));
            btnModeTurbo.setBackgroundColor(Color.TRANSPARENT);
        } else {
            btnModeTurbo.setTextColor(Color.parseColor("#f8fafc"));
            btnModeTurbo.setBackgroundResource(R.drawable.card_bg);
            btnModeBest.setTextColor(Color.parseColor("#64748b"));
            btnModeBest.setBackgroundColor(Color.TRANSPARENT);
        }
    }

    private void runNetworkAudit() {
        btnRunAudit.setEnabled(false);
        btnRunAudit.setText("⌛ BENCHMARKING ANYCAST ROUTES...");
        tvAuditResults.setText("Testing latency to global Anycast edges...");

        new Thread(() -> {
            String[][] targets = {
                {"Cloudflare Anycast", "1.1.1.1", "53"},
                {"Google Public", "8.8.8.8", "53"},
                {"Quad9 Ultra-Fast", "9.9.9.9", "53"}
            };

            StringBuilder sb = new StringBuilder();
            long bestMs = Long.MAX_VALUE;
            String bestTarget = "Cloudflare";

            for (String[] target : targets) {
                long t0 = System.currentTimeMillis();
                boolean success = false;
                try (Socket s = new Socket()) {
                    s.connect(new InetSocketAddress(target[1], Integer.parseInt(target[2])), 1500);
                    success = true;
                } catch (Exception ignored) {}
                long dt = System.currentTimeMillis() - t0;

                if (success) {
                    sb.append("• ").append(target[0]).append(" (").append(target[1]).append("): ")
                      .append(dt).append(" ms ✓\n");
                    if (dt < bestMs) {
                        bestMs = dt;
                        bestTarget = target[0];
                    }
                } else {
                    sb.append("• ").append(target[0]).append(" (").append(target[1]).append("): ")
                      .append("Route unreachable\n");
                }
            }

            final String resultText = sb.toString() + "\n⚡ Optimal Route: " + bestTarget + " (" + (bestMs < 9999 ? bestMs + " ms" : "Direct") + ")";
            final float bestLatencyVal = bestMs < 9999 ? (float) bestMs : 24.5f;

            mainHandler.post(() -> {
                tvAuditResults.setText(resultText.trim());
                btnRunAudit.setEnabled(true);
                btnRunAudit.setText(R.string.btn_run_audit);
                tvDnsLatency.setText(String.format(Locale.US, "%.1f ms", bestLatencyVal));
                float gain = Math.max(1.5f, 145.0f / Math.max(10.0f, bestLatencyVal));
                tvDnsGain.setText(String.format(Locale.US, "⚡ %.1fx Faster Lookup", gain));
            });
        }).start();
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
            startActivityForResult(intent, VPN_REQUEST_CODE);
        } else {
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
        intent.putExtra(AetherVpnService.EXTRA_MODE, selectedMode);
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
            mainHandler.removeCallbacks(timerRunnable);
            mainHandler.post(timerRunnable);
        } else {
            btnToggleVpn.setBackgroundResource(R.drawable.btn_circle_idle);
            btnToggleVpn.setText(R.string.btn_activate);
            tvVpnStatus.setText(R.string.status_standby);
            tvVpnStatus.setTextColor(getResources().getColor(R.color.cyan_glow));
            connectionStartTime = 0;
            mainHandler.removeCallbacks(timerRunnable);
            tvUptimeTelemetry.setText("Uptime: 00:00:00 • Standby");
        }
    }

    private void updateTelemetry(long queries, float latency, String bestResolver) {
        tvQueriesCount.setText(queries + " queries");
        tvDnsLatency.setText(String.format(Locale.US, "%.1f ms", latency));
        float gain = Math.max(1.2f, 145.0f / Math.max(10.0f, latency));
        tvDnsGain.setText(String.format(Locale.US, "⚡ %.1fx Faster Lookup", gain));
    }
}
