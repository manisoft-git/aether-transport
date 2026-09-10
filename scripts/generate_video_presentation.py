#!/usr/bin/env python3
"""
Aether-Transport: Autonomous Sub-IP Wireless Transport Engine
In-Depth Technical Presentation & MIT Portfolio Video Generator
Generates a broadcast-quality 720p 16:9 video with neural voiceover,
cybernetic glassmorphic slides, mathematical diagrams, Ken Burns motion,
and ambient electronic music bed.
"""

import os
import sys
import glob
import math
import subprocess
from PIL import Image, ImageDraw, ImageFont

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT_DIR = os.path.join(BASE_DIR, "static")
TEMP_DIR = "/tmp/aether_video_build"
FINAL_VIDEO = os.path.join(OUTPUT_DIR, "aether_transport_presentation.mp4")

os.makedirs(TEMP_DIR, exist_ok=True)
os.makedirs(OUTPUT_DIR, exist_ok=True)

FONT_BOLD = "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf"
FONT_REGULAR = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"
FONT_MONO = "/usr/share/fonts/truetype/dejavu/DejaVuSansMono-Bold.ttf"

# Video Canvas Dimensions (16:9 720p)
WIDTH = 1280
HEIGHT = 720

# Color Palette: Cybernetic Deep Space & Emerald/Cyan Quantum
BG_TOP = (7, 11, 20)          # #070b14 Deepest Space
BG_BOTTOM = (13, 21, 39)      # #0d1527 Dark Oceanic Navy
CARD_BG = (17, 26, 47, 230)   # Semi-transparent glass card
CARD_BORDER = (38, 70, 112)   # Sleek card edge
CYAN = (0, 229, 255)          # #00e5ff Electric Cyan
GREEN = (0, 255, 102)         # #00ff66 Quantum Emerald
AMBER = (245, 158, 11)        # #f59e0b Caution Amber
PURPLE = (168, 85, 247)       # #a855f7 Royal Violet
WHITE = (255, 255, 255)
TEXT_MUTED = (148, 163, 184)  # #94a3b8 Muted slate
RED = (239, 68, 68)           # #ef4444 Danger Coral

SCENES = [
    {
        "id": "scene1_crisis",
        "title": "01 // THE WIRELESS TRANSPORT CRISIS",
        "subtitle": "Equatorial rain fade & cellular multipath induce 15%–30% physical packet loss. Legacy TCP and QUIC misinterpret radio drops as buffer congestion, slashing transmission windows by 95% and collapsing throughput to a crawl.",
        "voice_text": "Across the Global South and equatorial regions like Nigeria, terrestrial 4G and 5G cellular networks face extreme physical volatility. During tropical convective storms, raindrop scattering and multipath reflections induce fifteen to thirty percent physical packet drops. Legacy protocols like TCP Cubic and QUIC misinterpret this radio noise as intermediate router congestion, slashing transmission windows by up to ninety-five percent. Even with fifty megabits of raw capacity, throughput collapses to a crawl."
    },
    {
        "id": "scene2_architecture",
        "title": "02 // INTRODUCING AETHER-TRANSPORT",
        "subtitle": "Engineered in memory-safe Rust by high school senior Manifest Subeno for the MIT Maker Portfolio (Class of 2027 / 2031). Aether decouples packet loss from congestion control, transforming lossy links into resilient data conduits.",
        "voice_text": "Meet Aether-Transport: an autonomous, sub-IP userspace wireless transport engine engineered in memory-safe Rust by high school senior Manifest Subeno. Designed as a capstone research project for the MIT Maker Portfolio and CSAIL Class of 2031, Aether decouples packet loss from congestion control, mathematically transforming lossy wireless links into resilient, high-speed data pipelines."
    },
    {
        "id": "scene3_fulcrum",
        "title": "03 // FULCRUM NETWORK CODING (GF(2) + GF(2^8))",
        "subtitle": "Inspired by Prof. Muriel Médard's research. Outer GF(2) binary expansion uses pure bitwise XORs for O(1) systematic decoding, paired with an inner GF(2^8) sliding-window random linear code for rateless repair. 80% less mobile energy.",
        "voice_text": "At the core of Aether is Fulcrum Network Coding, inspired by Professor Muriel Médard's seminal research. Dense Galois Field Gaussian elimination is computationally expensive for mobile processors. Aether solves this with a concatenated architecture: an outer binary expansion over GF 2 using pure bitwise XORs for instant systematic recovery, paired with an inner sliding-window random linear code over GF 2 to the 8th power for rateless repairs. This slashes decoding energy by eighty percent while eliminating retransmission pauses."
    },
    {
        "id": "scene4_ekf",
        "title": "04 // EKF DELAY-GRADIENT ARBITRATION & RF TELEMETRY",
        "subtitle": "An Extended Kalman Filter tracks queuing delay gradients (dΔ/dt). If packet loss occurs while dΔ/dt <= 0, queues are stable: proving the drop is physical radio noise. Aether maintains line pacing and proactively inflates repairs.",
        "voice_text": "How does Aether know whether a packet drop is caused by bufferbloat or physical rain fade? It employs an Extended Kalman Filter tracking queuing delay gradients: d Delta by dt. If packet drops occur while the delay gradient is non-positive, queues are stable, proving the drop was radio noise. Aether maintains transmission pacing and proactively inflates repair symbols. When the delay gradient turns positive, true congestion is detected, and pacing adjusts gracefully."
    },
    {
        "id": "scene5_pipeline",
        "title": "05 // HIGH-THROUGHPUT PIPELINE & HARDWARE ACCELERATION",
        "subtitle": "Zero-copy 16-byte fixed wire framing. Galois operations vectorized via ARM NEON and AVX2 4-bit nibble lookups (32 ops/instruction). eBPF/XDP kernel bypass and Subspace Homomorphic Vector MACs neutralize Byzantine pollution.",
        "voice_text": "To deliver line-rate throughput on low-power ARM smartphones and edge routers, Aether implements zero-copy sixteen-byte wire framing. Galois field multiplications are vectorized using ARM NEON and AVX2 four-bit nibble lookups, executing thirty-two field operations in a single instruction. Kernel overhead is eliminated using an eBPF XDP driver, while subspace homomorphic vector MACs defeat Byzantine packet pollution denial-of-service before solver ingestion."
    },
    {
        "id": "scene6_verification",
        "title": "06 // FORMAL VERIFICATION & EMPIRICAL BENCHMARKS",
        "subtitle": "Formally verified in TLA+ for deadlock-free state transitions and proven bounds-safe with Kani Rust. Under 15% packet loss, TCP Cubic collapses to 3.4 Mbps and QUIC to 14.8 Mbps. Aether sustains 62.8 Mbps — an 18.5x breakthrough.",
        "voice_text": "Aether is formally verified. Protocol state transitions are proven deadlock-free in TLA+, while Galois field solvers are verified bounds-safe using the Kani Rust model checker. In head-to-head empirical evaluations under fifteen percent packet loss and one hundred forty millisecond round-trip time, TCP Cubic collapses to three point four megabits per second, and QUIC reaches fourteen point eight. Aether powers through at sixty-two point eight megabits, an eighteen-fold performance breakthrough."
    },
    {
        "id": "scene7_vision",
        "title": "07 // MULTIPATH BONDING & PATH TO MIT (2027-2031)",
        "subtitle": "Asymmetric multipath bonding combines MTN + Airtel SIMs simultaneously. Transparent userspace IP tunneling. Manifest Subeno's journey proves transformative systems can emerge from Termux on Android. Ready for MIT Class of 2031.",
        "voice_text": "Aether supports asymmetric multipath bonding, simultaneously combining MTN and Airtel cellular links to eliminate coverage dead zones. Transparent userspace IP tunneling allows existing apps to run effortlessly over Aether. Manifest Subeno's journey proves that transformative systems engineering can emerge from an Android phone in Nigeria. The protocol specification, RFC draft, and open-source Rust crates stand ready for production deployment and the MIT Class of 2031."
    }
]

def draw_canvas_base():
    """Generates the background with sleek cybernetic grid lines and atmospheric ambient glows."""
    img = Image.new("RGB", (WIDTH, HEIGHT))
    draw = ImageDraw.Draw(img)
    
    # Gradient
    for y in range(HEIGHT):
        r_ratio = y / float(HEIGHT)
        r = int(BG_TOP[0] * (1 - r_ratio) + BG_BOTTOM[0] * r_ratio)
        g = int(BG_TOP[1] * (1 - r_ratio) + BG_BOTTOM[1] * r_ratio)
        b = int(BG_TOP[2] * (1 - r_ratio) + BG_BOTTOM[2] * r_ratio)
        draw.line([(0, y), (WIDTH, y)], fill=(r, g, b))
        
    # Grid lines
    grid_color = (20, 35, 60)
    for x in range(0, WIDTH, 64):
        draw.line([(x, 0), (x, HEIGHT)], fill=grid_color, width=1)
    for y in range(0, HEIGHT, 64):
        draw.line([(0, y), (WIDTH, y)], fill=grid_color, width=1)
        
    return img

def draw_header(draw, scene_title, tag_text="RESEARCH SPECIFICATION"):
    # Header bar
    draw.line([(60, 48), (WIDTH - 60, 48)], fill=(30, 58, 95), width=1)
    
    font_brand = ImageFont.truetype(FONT_BOLD, 15)
    font_tag = ImageFont.truetype(FONT_MONO, 11)
    
    draw.text((60, 24), "AETHER-TRANSPORT", fill=CYAN, font=font_brand)
    draw.text((230, 24), "// SOVEREIGN WIRELESS ENGINE", fill=TEXT_MUTED, font=font_brand)
    
    # Tag pill
    pw = 180
    px = WIDTH - 60 - pw
    draw.rounded_rectangle([px, 20, px + pw, 42], radius=4, fill=(15, 30, 54), outline=CYAN, width=1)
    draw.text((px + 12, 25), tag_text, fill=GREEN, font=font_tag)

def draw_subtitle_bar(draw, text):
    """Draws a clean, multi-line subtitle bar at the bottom of the video."""
    sy = 630
    draw.rounded_rectangle([60, sy, WIDTH - 60, sy + 68], radius=8, fill=(10, 16, 28), outline=(30, 50, 80), width=1)
    
    font_sub = ImageFont.truetype(FONT_REGULAR, 14)
    words = text.split()
    lines = []
    curr = ""
    for w in words:
        test = curr + " " + w if curr else w
        if len(test) < 115:
            curr = test
        else:
            lines.append(curr)
            curr = w
    if curr:
        lines.append(curr)
        
    for i, line in enumerate(lines[:3]):
        draw.text((80, sy + 10 + i * 20), line, fill=(241, 245, 249), font=font_sub)

def create_scene1():
    img = draw_canvas_base()
    draw = ImageDraw.Draw(img)
    draw_header(draw, SCENES[0]["title"], "PROBLEM: MATHIS COLLAPSE")
    
    font_title = ImageFont.truetype(FONT_BOLD, 26)
    font_h2 = ImageFont.truetype(FONT_BOLD, 16)
    font_body = ImageFont.truetype(FONT_REGULAR, 13)
    font_mono = ImageFont.truetype(FONT_MONO, 13)
    
    draw.text((60, 75), "01 // THE WIRELESS TRANSPORT CRISIS: RAIN FADE & MATHIS COLLAPSE", fill=CYAN, font=font_title)
    
    # Left Card: Physical Degradation
    draw.rounded_rectangle([60, 125, 620, 605], radius=12, fill=(13, 20, 36), outline=(38, 70, 112), width=1)
    draw.text((85, 145), "⛈️ Physical Atmospheric & Radio Fading", fill=AMBER, font=font_h2)
    
    left_items = [
        ("• Tropical Rain Attenuation:", "1-5mm raindrops match 1.8-3.5GHz carrier wavelengths."),
        ("• Severe Power Dropouts:", "Sudden 15 to 25 dB drops during convective downpours."),
        ("• Rayleigh Multipath Reflection:", "Destructive interference from buildings & corrugated roofs."),
        ("• Non-Congestive Frame Loss:", "10% to 30% physical drops with zero router buffer overflow.")
    ]
    for idx, (head, desc) in enumerate(left_items):
        y = 185 + idx * 80
        draw.text((85, y), head, fill=WHITE, font=font_h2)
        draw.text((85, y + 26), desc, fill=TEXT_MUTED, font=font_body)
        
    draw.rounded_rectangle([85, 515, 595, 580], radius=8, fill=(24, 16, 20), outline=RED, width=1)
    draw.text((105, 528), "STATUS: Extreme Physical Link Loss p = 0.15 (15%)", fill=RED, font=font_mono)
    draw.text((105, 552), "MTN / Airtel 4G LTE Nigerian Cellular Conditions", fill=TEXT_MUTED, font=font_body)

    # Right Card: Mathis Formula Collapse
    draw.rounded_rectangle([655, 125, WIDTH - 60, 605], radius=12, fill=(13, 20, 36), outline=(38, 70, 112), width=1)
    draw.text((680, 145), "📉 Classical TCP Throughput Collapse (Mathis Formula)", fill=RED, font=font_h2)
    
    draw.rounded_rectangle([680, 185, WIDTH - 85, 270], radius=8, fill=(7, 12, 22), outline=(30, 58, 95), width=1)
    draw.text((700, 200), "Mathis Steady-State Upper Bound:", fill=TEXT_MUTED, font=font_body)
    draw.text((700, 225), "Throughput <= (MSS / (RTT * sqrt(p))) * 1.225", fill=CYAN, font=font_mono)
    
    right_items = [
        ("❌ False Congestion Diagnosis:", "TCP Cubic assumes packet loss == router buffer queue saturation."),
        ("✂️ Catastrophic Window Cut:", "TCP slashes congestion window (cwnd) by 50% on every drop."),
        ("🐌 Throughput Plunge:", "50 Mbps link collapses to 264 Kbps (200x performance penalty!)."),
        ("⏳ Head-of-Line (HoL) Blocking:", "QUIC and TCP stall subsequent packets waiting for retransmissions.")
    ]
    for idx, (head, desc) in enumerate(right_items):
        y = 295 + idx * 68
        draw.text((680, y), head, fill=WHITE, font=font_h2)
        draw.text((680, y + 24), desc, fill=TEXT_MUTED, font=font_body)
        
    draw_subtitle_bar(draw, SCENES[0]["subtitle"])
    return img

def create_scene2():
    img = draw_canvas_base()
    draw = ImageDraw.Draw(img)
    draw_header(draw, SCENES[1]["title"], "SOVEREIGN SYSTEM ARCHITECTURE")
    
    font_title = ImageFont.truetype(FONT_BOLD, 26)
    font_h2 = ImageFont.truetype(FONT_BOLD, 16)
    font_body = ImageFont.truetype(FONT_REGULAR, 13)
    font_mono = ImageFont.truetype(FONT_MONO, 13)
    
    draw.text((60, 75), "02 // INTRODUCING AETHER-TRANSPORT: SOVEREIGN PROTOCOL SUITE", fill=CYAN, font=font_title)
    
    # Left Card: Candidate & MIT Portfolio
    draw.rounded_rectangle([60, 125, 480, 605], radius=12, fill=(13, 20, 36), outline=(38, 70, 112), width=1)
    draw.text((85, 145), "🎓 MIT Maker Portfolio Capstone", fill=GREEN, font=font_h2)
    
    prof_items = [
        ("Architect:", "Manifest Subeno"),
        ("Education:", "High School Senior (SS3), Good Shepherd College"),
        ("Location:", "Port Harcourt / Delta State, Nigeria"),
        ("Timeline:", "Graduating 2027 • Target: MIT Class of 2031"),
        ("Department:", "MIT EECS / CSAIL / RLE Research Sponsor"),
        ("Language:", "100% Memory-Safe Systems Rust (2021 Edition)"),
        ("Scope:", "13 Specialized Rust Crates + IETF RFC Draft")
    ]
    for idx, (k, v) in enumerate(prof_items):
        y = 190 + idx * 48
        draw.text((85, y), k, fill=CYAN, font=font_h2)
        draw.text((85, y + 22), v, fill=WHITE, font=font_body)
        
    draw.rounded_rectangle([85, 530, 455, 585], radius=6, fill=(15, 35, 60), outline=CYAN, width=1)
    draw.text((100, 548), "⭐ 18.5x Throughput Advantage Over TCP Cubic", fill=WHITE, font=font_mono)

    # Right Card: Multi-Layer Protocol Stack
    draw.rounded_rectangle([515, 125, WIDTH - 60, 605], radius=12, fill=(13, 20, 36), outline=(38, 70, 112), width=1)
    draw.text((540, 145), "🏗️ Autonomous Multi-Layer Sub-IP Architecture", fill=CYAN, font=font_h2)
    
    layers = [
        ("APPLICATION LAYER", "Web Browsers • HTTP/3 • Video Streaming • Games • DNS", (30, 45, 75), CYAN),
        ("AETHER USERSPACE TUN", "aether-tun: Virtual TUN interface (tun0) routing raw IPv4/IPv6", (20, 35, 60), GREEN),
        ("FULCRUM NETWORK CODER", "aether-coder: GF(2) Outer Expansion + GF(2^8) Sliding-Window RLNC", (15, 45, 50), GREEN),
        ("EKF CONGESTION ARBITER", "aether-congestion: Delay gradient dΔ/dt + Cellular Modem Telemetry", (45, 35, 20), AMBER),
        ("CRYPTOGRAPHIC MAC SENTINEL", "aether-crypto: Subspace Homomorphic Vector MACs against Byzantine Poison", (40, 20, 50), PURPLE),
        ("HARDWARE KERNEL BYPASS", "aether-xdp: Zero-copy eBPF/XDP driver directly to physical NIC / LTE modem", (25, 25, 45), CYAN)
    ]
    for idx, (l_title, l_desc, bg_col, edge_col) in enumerate(layers):
        ly = 185 + idx * 68
        draw.rounded_rectangle([540, ly, WIDTH - 85, ly + 58], radius=8, fill=bg_col, outline=edge_col, width=1)
        draw.text((558, ly + 8), l_title, fill=edge_col, font=font_mono)
        draw.text((558, ly + 30), l_desc, fill=TEXT_MUTED, font=font_body)
        
    draw_subtitle_bar(draw, SCENES[1]["subtitle"])
    return img

def create_scene3():
    img = draw_canvas_base()
    draw = ImageDraw.Draw(img)
    draw_header(draw, SCENES[2]["title"], "ALGEBRAIC CODING INNOVATION")
    
    font_title = ImageFont.truetype(FONT_BOLD, 26)
    font_h2 = ImageFont.truetype(FONT_BOLD, 16)
    font_body = ImageFont.truetype(FONT_REGULAR, 13)
    font_mono = ImageFont.truetype(FONT_MONO, 13)
    
    draw.text((60, 75), "03 // FULCRUM NETWORK CODING: CONCATENATING GF(2) & GF(2^8)", fill=CYAN, font=font_title)
    
    # Left Card: Concatenated Architecture
    draw.rounded_rectangle([60, 125, 630, 605], radius=12, fill=(13, 20, 36), outline=(38, 70, 112), width=1)
    draw.text((85, 145), "⚡ Médard-Lucani Fulcrum Concatenation", fill=GREEN, font=font_h2)
    
    steps = [
        ("Step 1: Source Packets (K)", "K payload packets are prepared as vectors over finite field GF(2^8)."),
        ("Step 2: Outer GF(2) Expansion (K + r)", "Generates r extra packets using pure bitwise XORs (O(K·S) complexity)."),
        ("Step 3: Inner GF(2^8) Sliding Window", "Generates rateless random linear combinations for loss repair."),
        ("Step 4: Fast Systematic Decoding", "Systematic packets decoded via back-substitution with zero matrix inversion!")
    ]
    for idx, (st, sd) in enumerate(steps):
        sy = 190 + idx * 80
        draw.rounded_rectangle([85, sy, 605, sy + 68], radius=8, fill=(18, 28, 48), outline=(30, 58, 95), width=1)
        draw.text((100, sy + 10), st, fill=CYAN, font=font_h2)
        draw.text((100, sy + 34), sd, fill=TEXT_MUTED, font=font_body)
        
    draw.rounded_rectangle([85, 525, 605, 585], radius=8, fill=(15, 35, 30), outline=GREEN, width=1)
    draw.text((100, 545), "🔋 80% Energy Reduction on Mobile Microprocessors", fill=GREEN, font=font_mono)

    # Right Card: Mathematical Superiority
    draw.rounded_rectangle([660, 125, WIDTH - 60, 605], radius=12, fill=(13, 20, 36), outline=(38, 70, 112), width=1)
    draw.text((685, 145), "📐 Finite Field Mathematical Proofs", fill=CYAN, font=font_h2)
    
    math_proofs = [
        ("Irreducible Field Polynomial:", "P(x) = x^8 + x^4 + x^3 + x + 1  (Hex: 0x11D / Rijndael)"),
        ("O(1) Branchless Field Multiplication:", "a * b = EXP_TABLE[(LOG_TABLE[a] + LOG_TABLE[b]) % 255]"),
        ("Singularity Probability Bound:", "Pr(Singular) <= 1 / (q - 1) = 1 / 255 ≈ 0.39%"),
        ("Erasure Guarantee:", "ANY K linear combinations guarantee complete block recovery!")
    ]
    for idx, (mt, md) in enumerate(math_proofs):
        my = 195 + idx * 75
        draw.text((685, my), mt, fill=WHITE, font=font_h2)
        draw.text((685, my + 24), md, fill=CYAN, font=font_mono)
        
    draw.rounded_rectangle([685, 510, WIDTH - 85, 585], radius=8, fill=(10, 24, 40), outline=CYAN, width=1)
    draw.text((705, 525), "ZERO RETRANSMISSION DELAYS", fill=WHITE, font=font_h2)
    draw.text((705, 550), "Eliminates Head-of-Line Blocking completely in video and voice streams.", fill=TEXT_MUTED, font=font_body)
    
    draw_subtitle_bar(draw, SCENES[2]["subtitle"])
    return img

def create_scene4():
    img = draw_canvas_base()
    draw = ImageDraw.Draw(img)
    draw_header(draw, SCENES[3]["title"], "CONGESTION CONTROL RADAR")
    
    font_title = ImageFont.truetype(FONT_BOLD, 26)
    font_h2 = ImageFont.truetype(FONT_BOLD, 16)
    font_body = ImageFont.truetype(FONT_REGULAR, 13)
    font_mono = ImageFont.truetype(FONT_MONO, 13)
    
    draw.text((60, 75), "04 // EKF DELAY-GRADIENT ARBITRATION & CROSS-LAYER MODEM SENSING", fill=CYAN, font=font_title)
    
    # Left Card: Extended Kalman Filter Delay-Gradient
    draw.rounded_rectangle([60, 125, 630, 605], radius=12, fill=(13, 20, 36), outline=(38, 70, 112), width=1)
    draw.text((85, 145), "🧠 Extended Kalman Filter Queue Observer", fill=AMBER, font=font_h2)
    
    draw.rounded_rectangle([85, 185, 605, 260], radius=8, fill=(10, 18, 32), outline=(30, 58, 95), width=1)
    draw.text((105, 200), "Queuing Delay Metric:", fill=TEXT_MUTED, font=font_body)
    draw.text((105, 225), "Δ(t) = RTT(t) - D_min    Delay Gradient: dΔ/dt", fill=CYAN, font=font_mono)
    
    ekf_rules = [
        ("dΔ/dt <= 0 AND Packet Drop Occurs:", "Queues are EMPTYING/STABLE! Drop is 100% physical radio rain fade.\n--> MAINTAIN line pacing. Proactively inflate Fulcrum repair rate!"),
        ("dΔ/dt > 0 AND RTT Balloons:", "Bottleneck router buffer is filling! True bufferbloat detected.\n--> PUSH RATE DOWN smoothly without inducing packet collapse.")
    ]
    for idx, (rt, rd) in enumerate(ekf_rules):
        ry = 285 + idx * 115
        box_edge = GREEN if idx == 0 else AMBER
        draw.rounded_rectangle([85, ry, 605, ry + 100], radius=8, fill=(18, 26, 44), outline=box_edge, width=1)
        draw.text((100, ry + 12), rt, fill=box_edge, font=font_h2)
        draw.text((100, ry + 38), rd, fill=WHITE, font=font_body)

    # Right Card: Cellular Modem Telemetry (RSRP, SINR)
    draw.rounded_rectangle([660, 125, WIDTH - 60, 605], radius=12, fill=(13, 20, 36), outline=(38, 70, 112), width=1)
    draw.text((685, 145), "📡 Real-Time Physical Layer Coupling (aether-telemetry)", fill=CYAN, font=font_h2)
    
    rf_metrics = [
        ("RSRP (Reference Signal Received Power):", "Measures cell tower radio distance and physical attenuation in dBm."),
        ("RSRQ (Reference Signal Received Quality):", "Quantifies carrier RSSI versus thermal receiver noise."),
        ("SINR (Signal-to-Interference-plus-Noise):", "Determines physical modulation and coding scheme (MCS) capability.")
    ]
    for idx, (rt, rd) in enumerate(rf_metrics):
        ry = 195 + idx * 85
        draw.text((685, ry), rt, fill=WHITE, font=font_h2)
        draw.text((685, ry + 24), rd, fill=TEXT_MUTED, font=font_body)
        
    draw.rounded_rectangle([685, 475, WIDTH - 85, 585], radius=8, fill=(15, 30, 50), outline=CYAN, width=1)
    draw.text((705, 495), "PREDICTIVE ADAPTIVE FEC INJECTION", fill=GREEN, font=font_h2)
    draw.text((705, 525), "When SINR drops below 0 dB, Aether injects repair symbols\nBEFORE the transport layer experiences packet starvation!", fill=WHITE, font=font_body)
    
    draw_subtitle_bar(draw, SCENES[3]["subtitle"])
    return img

def create_scene5():
    img = draw_canvas_base()
    draw = ImageDraw.Draw(img)
    draw_header(draw, SCENES[4]["title"], "HIGH-THROUGHPUT ENGINE")
    
    font_title = ImageFont.truetype(FONT_BOLD, 26)
    font_h2 = ImageFont.truetype(FONT_BOLD, 16)
    font_body = ImageFont.truetype(FONT_REGULAR, 13)
    font_mono = ImageFont.truetype(FONT_MONO, 13)
    
    draw.text((60, 75), "05 // HIGH-THROUGHPUT SYSTEMS PIPELINE: SIMD, XDP & HOMOMORPHIC MACS", fill=CYAN, font=font_title)
    
    # Left Card: SIMD & Zero-Copy Framing
    draw.rounded_rectangle([60, 125, 630, 605], radius=12, fill=(13, 20, 36), outline=(38, 70, 112), width=1)
    draw.text((85, 145), "⚡ Zero-Copy SIMD Vectorization Engine", fill=CYAN, font=font_h2)
    
    simd_items = [
        ("16-Byte Compact Wire Header:", "Magic(2B) • Type(1B) • Flags(1B) • Session(4B) • Window(4B) • Coeff(4B)"),
        ("Zero Heap Allocations:", "Header parsed via direct slice pointer casting (zero-copy memory bus)."),
        ("AVX2 256-Bit Vectorization:", "Uses _mm256_shuffle_epi8 to execute 32 field multiplications per cycle!"),
        ("ARM NEON Vector Acceleration:", "vtbl1_u8 lookup tables optimized for Android smartphones & IoT chips.")
    ]
    for idx, (st, sd) in enumerate(simd_items):
        sy = 190 + idx * 80
        draw.rounded_rectangle([85, sy, 605, sy + 68], radius=8, fill=(18, 28, 48), outline=(30, 58, 95), width=1)
        draw.text((100, sy + 10), st, fill=WHITE, font=font_h2)
        draw.text((100, sy + 34), sd, fill=CYAN, font=font_mono)
        
    draw.rounded_rectangle([85, 525, 605, 585], radius=8, fill=(10, 35, 45), outline=CYAN, width=1)
    draw.text((100, 545), "🚀 180+ MB/s Real-Time Decoding on Low-Power ARM CPUs", fill=WHITE, font=font_mono)

    # Right Card: eBPF/XDP & Byzantine Defense
    draw.rounded_rectangle([660, 125, WIDTH - 60, 605], radius=12, fill=(13, 20, 36), outline=(38, 70, 112), width=1)
    draw.text((685, 145), "🛡️ Kernel Bypass & Byzantine Defense", fill=PURPLE, font=font_h2)
    
    right_items = [
        ("eBPF/XDP Driver (aether-xdp):", "Ingests packets at network driver level before Linux sk_buff allocation."),
        ("The Byzantine Pollution Threat:", "In linear coding, 1 forged byte corrupts the entire Gaussian elimination!"),
        ("Subspace Homomorphic Vector MACs:", "Tag(α u + β v) = α Tag(u) + β Tag(v) mod q"),
        ("Zero-Cost Pre-Filter:", "Validates packet integrity before inserting into the Gaussian solver.")
    ]
    for idx, (rt, rd) in enumerate(right_items):
        ry = 195 + idx * 85
        draw.text((685, ry), rt, fill=WHITE, font=font_h2)
        draw.text((685, ry + 24), rd, fill=TEXT_MUTED, font=font_body)
        
    draw.rounded_rectangle([685, 510, WIDTH - 85, 585], radius=8, fill=(25, 15, 35), outline=PURPLE, width=1)
    draw.text((705, 525), "IRONCLAD PACKET IMMUNITY", fill=PURPLE, font=font_h2)
    draw.text((705, 550), "Malformed or adversarial packets discarded in O(1) time.", fill=TEXT_MUTED, font=font_body)
    
    draw_subtitle_bar(draw, SCENES[4]["subtitle"])
    return img

def create_scene6():
    img = draw_canvas_base()
    draw = ImageDraw.Draw(img)
    draw_header(draw, SCENES[5]["title"], "EMPIRICAL BENCHMARKS")
    
    font_title = ImageFont.truetype(FONT_BOLD, 26)
    font_h2 = ImageFont.truetype(FONT_BOLD, 16)
    font_body = ImageFont.truetype(FONT_REGULAR, 13)
    font_mono = ImageFont.truetype(FONT_MONO, 13)
    
    draw.text((60, 75), "06 // FORMAL VERIFICATION & HEAD-TO-HEAD BENCHMARK SHOOTOUT", fill=CYAN, font=font_title)
    
    # Left Card: Formal Verification
    draw.rounded_rectangle([60, 125, 520, 605], radius=12, fill=(13, 20, 36), outline=(38, 70, 112), width=1)
    draw.text((85, 145), "🔬 Rigorous Mathematical Verification", fill=GREEN, font=font_h2)
    
    proofs = [
        ("TLA+ Protocol Model (spec/AetherProtocol.tla):", "Proven deadlock-free via TLC model checker across all out-of-order packet permutations."),
        ("Kani Rust Model Checker (kani/):", "Formally proves finite field solvers free of panics, buffer overflows, and arithmetic faults."),
        ("Zero-Defect Verification Suite:", "5-layer automated test harness ensuring zero regressions across all fleet submodules.")
    ]
    for idx, (pt, pd) in enumerate(proofs):
        py = 195 + idx * 110
        draw.rounded_rectangle([85, py, 495, py + 95], radius=8, fill=(15, 26, 42), outline=(30, 58, 95), width=1)
        draw.text((100, py + 12), pt, fill=GREEN, font=font_h2)
        draw.text((100, py + 38), pd, fill=TEXT_MUTED, font=font_body)
        
    draw.rounded_rectangle([85, 535, 495, 585], radius=6, fill=(12, 36, 24), outline=GREEN, width=1)
    draw.text((100, 552), "✓ TLA+ & KANI FORMAL PROOFS PASSING", fill=WHITE, font=font_mono)

    # Right Card: Benchmark Bar Chart (15% Loss)
    draw.rounded_rectangle([550, 125, WIDTH - 60, 605], radius=12, fill=(13, 20, 36), outline=(38, 70, 112), width=1)
    draw.text((575, 145), "📊 15% Packet Loss Shootout (140ms RTT Gilbert-Elliott)", fill=CYAN, font=font_h2)
    
    bars = [
        ("TCP Cubic (Standard Web)", 3.4, 62.8, RED, "3.4 Mbps (Collapsed)"),
        ("TCP BBR (Model-Based)", 8.1, 62.8, AMBER, "8.1 Mbps (Fluctuating)"),
        ("Baseline QUIC (HTTP/3)", 14.8, 62.8, PURPLE, "14.8 Mbps (SACK Storm)"),
        ("AETHER-TRANSPORT", 62.8, 62.8, GREEN, "62.8 Mbps (18.5x SPEEDUP!)")
    ]
    for idx, (label, val, max_val, bar_col, speed_str) in enumerate(bars):
        by = 210 + idx * 85
        draw.text((575, by), label, fill=WHITE, font=font_h2)
        draw.text((WIDTH - 260, by), speed_str, fill=bar_col, font=font_mono)
        
        # Bar track
        track_w = WIDTH - 60 - 575 - 20
        draw.rounded_rectangle([575, by + 28, 575 + track_w, by + 52], radius=6, fill=(20, 30, 48))
        filled_w = max(15, int((val / max_val) * track_w))
        draw.rounded_rectangle([575, by + 28, 575 + filled_w, by + 52], radius=6, fill=bar_col)
        
    draw.rounded_rectangle([575, 530, WIDTH - 85, 585], radius=8, fill=(10, 40, 30), outline=GREEN, width=1)
    draw.text((595, 548), "⚡ Aether: 4.2x Faster than QUIC • 18.5x Faster than TCP", fill=GREEN, font=font_h2)
    
    draw_subtitle_bar(draw, SCENES[5]["subtitle"])
    return img

def create_scene7():
    img = draw_canvas_base()
    draw = ImageDraw.Draw(img)
    draw_header(draw, SCENES[6]["title"], "THE ROAD TO MIT 2027–2031")
    
    font_title = ImageFont.truetype(FONT_BOLD, 26)
    font_h2 = ImageFont.truetype(FONT_BOLD, 16)
    font_body = ImageFont.truetype(FONT_REGULAR, 13)
    font_mono = ImageFont.truetype(FONT_MONO, 13)
    
    draw.text((60, 75), "07 // DUAL-SIM CELLULAR BONDING & THE JOURNEY TO MIT EECS / CSAIL", fill=CYAN, font=font_title)
    
    # Left Card: Multipath Telco Bonding
    draw.rounded_rectangle([60, 125, 600, 605], radius=12, fill=(13, 20, 36), outline=(38, 70, 112), width=1)
    draw.text((85, 145), "📶 Asymmetric Multi-Telco Bonding", fill=GREEN, font=font_h2)
    
    mp_items = [
        ("SIM 1 (MTN 4G LTE):", "Delivers 25 Mbps systematic packet stream on primary path."),
        ("SIM 2 (Airtel 4G LTE):", "Delivers 20 Mbps repair packet stream on secondary path."),
        ("Seamless Packet Aggregation:", "Bonds two distinct cellular interfaces into a unified 45 Mbps pipe."),
        ("Zero-Downtime Failover:", "If MTN drops out completely, Airtel absorbs the session instantly!")
    ]
    for idx, (mt, md) in enumerate(mp_items):
        my = 195 + idx * 78
        draw.rounded_rectangle([85, my, 575, my + 68], radius=8, fill=(16, 26, 42), outline=(30, 58, 95), width=1)
        draw.text((100, my + 10), mt, fill=CYAN, font=font_h2)
        draw.text((100, my + 34), md, fill=WHITE, font=font_body)
        
    draw.rounded_rectangle([85, 525, 575, 585], radius=8, fill=(15, 35, 25), outline=GREEN, width=1)
    draw.text((105, 545), "🚀 100% Uninterrupted VoIP, Video Calls & Cloud Gaming", fill=GREEN, font=font_mono)

    # Right Card: The Candidate Journey
    draw.rounded_rectangle([630, 125, WIDTH - 60, 605], radius=12, fill=(13, 20, 36), outline=(38, 70, 112), width=1)
    draw.text((655, 145), "🏛️ Candidate Mission & MIT Aspirations", fill=CYAN, font=font_h2)
    
    cand_items = [
        ("Coded on Android Phone via Termux:", "Engineered amidst power outages and mobile constraints in Nigeria."),
        ("Academic Research Artifacts:", "NSDI-style Technical Preprint • IETF RFC Draft • 10-Chapter Textbook"),
        ("Undergraduate Application:", "Graduating 2027 • Applying to MIT EECS / CSAIL (Class of 2031)"),
        ("Target Research Groups:", "Prof. Muriel Médard (RLE) • Prof. Hari Balakrishnan (CSAIL)")
    ]
    for idx, (ct, cd) in enumerate(cand_items):
        cy = 195 + idx * 78
        draw.rounded_rectangle([655, cy, WIDTH - 85, cy + 68], radius=8, fill=(16, 26, 42), outline=(30, 58, 95), width=1)
        draw.text((670, cy + 10), ct, fill=AMBER if idx == 0 else WHITE, font=font_h2)
        draw.text((670, cy + 34), cd, fill=TEXT_MUTED, font=font_body)
        
    draw.rounded_rectangle([655, 525, WIDTH - 85, 585], radius=8, fill=(15, 30, 55), outline=CYAN, width=1)
    draw.text((675, 545), "🌟 Manifest Subeno // Future MIT Systems Researcher", fill=CYAN, font=font_mono)
    
    draw_subtitle_bar(draw, SCENES[6]["subtitle"])
    return img

def render_slides():
    """Renders all 7 scenes to PNG."""
    gens = [
        ("scene1_crisis", create_scene1),
        ("scene2_architecture", create_scene2),
        ("scene3_fulcrum", create_scene3),
        ("scene4_ekf", create_scene4),
        ("scene5_pipeline", create_scene5),
        ("scene6_verification", create_scene6),
        ("scene7_vision", create_scene7)
    ]
    for sid, gen_fn in gens:
        path = os.path.join(TEMP_DIR, f"{sid}.png")
        img = gen_fn()
        img.save(path, quality=95)
        print(f"  [✓] Rendered slide: {sid}.png")

def generate_voiceovers():
    """Generates crystal-clear neural TTS audio for each scene using edge-tts."""
    for idx, sc in enumerate(SCENES):
        sid = sc["id"]
        out_mp3 = os.path.join(TEMP_DIR, f"{sid}.mp3")
        text = sc["voice_text"]
        print(f"  [*] Synthesizing neural voiceover {idx+1}/7: {sid}...")
        cmd = [
            "edge-tts",
            "--voice", "en-US-ChristopherNeural",
            "--text", text,
            "--write-media", out_mp3
        ]
        subprocess.run(cmd, check=True)

def generate_scene_clip(scene, index):
    """Encodes a single scene video clip with Ken Burns motion and synced audio."""
    sid = scene["id"]
    img_path = os.path.join(TEMP_DIR, f"{sid}.png")
    audio_path = os.path.join(TEMP_DIR, f"{sid}.mp3")
    out_clip = os.path.join(TEMP_DIR, f"clip_{index:02d}.mp4")
    
    res = subprocess.run([
        "ffprobe", "-i", audio_path, "-show_entries", "format=duration", "-v", "quiet", "-of", "csv=p=0"
    ], capture_output=True, text=True, check=True)
    duration = float(res.stdout.strip())
    
    total_frames = int(duration * 25)
    filter_complex = (
        f"[0:v]scale=1280:720,zoompan=z='min(zoom+0.00025,1.035)':d={total_frames}:x='iw/2-(iw/zoom/2)':y='ih/2-(ih/zoom/2)':s=1280x720:fps=25[v]"
    )
    
    cmd = [
        "ffmpeg", "-y",
        "-loop", "1", "-i", img_path,
        "-i", audio_path,
        "-filter_complex", filter_complex,
        "-map", "[v]", "-map", "1:a",
        "-c:v", "libx264", "-preset", "ultrafast", "-crf", "22", "-pix_fmt", "yuv420p",
        "-c:a", "aac", "-b:a", "192k",
        "-t", str(duration),
        out_clip
    ]
    subprocess.run(cmd, check=True)
    print(f"  [✓] Encoded clip {index+1}/7: {out_clip} ({duration:.1f}s)")
    return out_clip, duration

def generate_ambient_soundtrack(duration):
    """Synthesizes a futuristic ambient synthesizer bed via ffmpeg."""
    soundtrack = os.path.join(TEMP_DIR, "ambient_soundtrack.wav")
    print(f"  [*] Synthesizing sci-fi ambient electronic soundtrack ({duration:.1f}s)...")
    cmd = [
        "ffmpeg", "-y",
        "-f", "lavfi",
        "-i", f"aevalsrc="
              f"sin(110.00*2*PI*t)*0.025*sin(2*PI*t/12) + "
              f"sin(146.83*2*PI*t)*0.020*cos(2*PI*t/12) + "
              f"sin(164.81*2*PI*t)*0.018*sin(2*PI*t/12) + "
              f"sin(220.00*2*PI*t)*0.015:s=44100:d={duration}",
        "-af", "lowpass=f=900,volume=0.25",
        soundtrack
    ]
    subprocess.run(cmd, check=True)
    return soundtrack

def assemble_final_video(clips, total_duration, music_path):
    """Stitches all video clips, mixes audio, and exports final broadcast MP4."""
    concat_list = os.path.join(TEMP_DIR, "concat.txt")
    with open(concat_list, "w") as f:
        for c in clips:
            f.write(f"file '{c}'\n")
            
    raw_video = os.path.join(TEMP_DIR, "raw_video.mp4")
    print("  [*] Concatenating scene video streams...")
    subprocess.run([
        "ffmpeg", "-y",
        "-f", "concat", "-safe", "0",
        "-i", concat_list,
        "-c", "copy",
        raw_video
    ], check=True)
    
    print("  [*] Audio mastering & final MP4 export...")
    cmd = [
        "ffmpeg", "-y",
        "-i", raw_video,
        "-i", music_path,
        "-filter_complex",
        "[0:a]volume=1.0[v_aud];"
        "[1:a]volume=0.15,afade=t=out:st=" + str(total_duration - 3) + ":d=3[bg_music];"
        "[v_aud][bg_music]amix=inputs=2:duration=first:dropout_transition=3[a_out]",
        "-map", "0:v", "-map", "[a_out]",
        "-c:v", "copy",
        "-c:a", "aac", "-b:a", "192k",
        "-movflags", "+faststart",
        FINAL_VIDEO
    ]
    subprocess.run(cmd, check=True)
    print(f"🎉 Final broadcast presentation exported to: {FINAL_VIDEO}")

def main():
    print("=" * 70)
    print(" AETHER-TRANSPORT: IN-DEPTH TECHNICAL PRESENTATION VIDEO GENERATOR")
    print("=" * 70)
    
    print("[1/5] Rendering High-Definition Cybernetic Slide Graphics...")
    render_slides()
    
    print("[2/5] Synthesizing Deep Neural Voiceovers with Edge-TTS...")
    generate_voiceovers()
    
    print("[3/5] Encoding Video Clips with Ken Burns Motion...")
    clips = []
    total_dur = 0.0
    for idx, sc in enumerate(SCENES):
        clip, dur = generate_scene_clip(sc, idx)
        clips.append(clip)
        total_dur += dur
        
    print(f"[4/5] Synthesizing Ambient Electronic Soundscape ({total_dur:.1f}s)...")
    music = generate_ambient_soundtrack(total_dur)
    
    print("[5/5] Final Broadcast Mastering & MP4 Export...")
    assemble_final_video(clips, total_dur, music)
    
    fsize = os.path.getsize(FINAL_VIDEO) / (1024 * 1024)
    print("=" * 70)
    print(f" [SUCCESS] Output File: {FINAL_VIDEO}")
    print(f" [SUCCESS] Resolution:  1280x720 (16:9 HD)")
    print(f" [SUCCESS] Total Time:  {total_dur:.1f} seconds ({total_dur/60:.1f} minutes)")
    print(f" [SUCCESS] File Size:   {fsize:.2f} MB")
    print("=" * 70)

if __name__ == "__main__":
    main()
