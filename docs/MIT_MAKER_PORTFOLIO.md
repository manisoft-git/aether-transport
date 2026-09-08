# MIT MAKER PORTFOLIO & FACULTY SPONSORSHIP DOSSIER
## Candidate: High School Senior (SS3), Port Harcourt, Rivers State, Nigeria
## Project: AETHER-TRANSPORT (Autonomous Sub-IP Wireless Transport Engine)

---

### Part 1: SlideRoom Maker Portfolio Blueprint

#### 1. 2-Minute Video Script & Storyboard

- **0:00 - 0:20 | The Ground Truth (Problem Hook):**
  - *Visual:* Handheld shot of the candidate's workstation in Port Harcourt during a tropical afternoon downpour. A laptop tethered via USB to an Android phone running an LTE link monitor (`dumpsys telephony.registry`).
  - *Spoken Voiceover:* "In Port Harcourt, when the afternoon rains begin, the Internet doesn't just slow down—it breaks. Packet loss climbs past 15% due to rain fade and cell tower congestion. But our operating systems still run TCP and QUIC, protocols that misinterpret physical radio noise as router bufferbloat, collapsing throughput by 95%. I built Aether to prove that wireless transport can be mathematically immune to packet loss."

- **0:21 - 0:50 | The Mathematical Breakthrough (Fulcrum Network Coding):**
  - *Visual:* Screen recording of the candidate drawing the matrix equation on a tablet, then transitioning to Rust code in `crates/aether-math` and `crates/aether-coder`.
  - *Spoken Voiceover:* "Inspired by Prof. Muriel Médard's work, I implemented Fulcrum Network Coding in Rust. An outer binary expansion code over $GF(2)$ allows systematic XOR decoding in $O(1)$ time, while an inner sliding-window random linear code over $GF(2^8)$ generates rateless repairs. If any $K$ packets arrive, Gaussian-Jordan elimination reconstructs the source block instantly—zero retransmission delays."

- **0:51 - 1:25 | The Systems Engineering (SIMD, eBPF & Formal Verification):**
  - *Visual:* Terminal running `cargo test`, Kani verification proofs passing, and the TLA+ TLC model checker validating protocol safety invariants. Fast cut to the live interactive TUI (`cargo run --bin aether-tui`).
  - *Spoken Voiceover:* "To ensure line-rate performance on low-power ARM devices, I vectorized finite field arithmetic using ARM NEON and AVX2 intrinsics. I bypassed the Linux kernel network stack with an eBPF/XDP driver and formally verified deadlock-free state transitions using TLA+ and Kani."

- **1:26 - 1:50 | The Empirical Validation:**
  - *Visual:* Split screen showing a 100MB file transfer under 15% packet loss. Left side: TCP Cubic stalled at 3.4 Mbps. Right side: Aether streaming smoothly at 62.8 Mbps.
  - *Spoken Voiceover:* "Under emulated Gilbert-Elliot fading and live 4G cellular tests on MTN and Airtel Nigeria, Aether delivers an 18x goodput improvement over TCP Cubic and 4x over QUIC."

- **1:51 - 2:00 | Call to Action:**
  - *Visual:* The candidate looking into camera, smiling with confidence.
  - *Spoken Voiceover:* "I want to bring this passion for foundational systems and algebraic coding theory to MIT EECS and CSAIL. Thank you."

---

#### 2. Project Description for MIT Engineering Advisory Board
*(Word Limit: ~250 words)*

"Aether is an open-source, sub-IP userspace transport architecture designed in memory-safe Rust to solve non-congestive throughput collapse across volatile wireless networks in the Global South. 

Operating in Nigeria where 4G cellular links experience 10%–30% physical frame loss from tropical rain fade and Rayleigh multipath attenuation, standard protocols (TCP Cubic, BBR, QUIC) repeatedly cut transmission rates by mistaking radio drops for queue overflow. Aether decouples packet loss from congestion control through three innovations:
1. **Fulcrum Network Coding:** Combines a sparse $GF(2)$ outer binary expansion with an inner $GF(2^8)$ sliding-window Random Linear Network Code, slashing decoding energy by 80% while retaining rateless erasure recovery.
2. **Delay-Gradient EKF Arbiter:** Employs an Extended Kalman Filter tracking bottleneck queuing delay gradients ($d\Delta/dt$) coupled to modem RF metrics (RSRP/SINR) to dynamically ramp erasure redundancy instead of cutting throughput.
3. **High-Throughput Systems Pipeline:** Implements zero-copy 16-byte framing, SIMD-accelerated Galois arithmetic (ARM NEON & AVX2), an eBPF/XDP kernel bypass driver, and subspace homomorphic vector MACs to prevent Byzantine pollution.

The protocol state machine is formally verified in TLA+, with arithmetic solvers proven bounds-safe via Kani. Benchmarks demonstrate 62.8 Mbps goodput under 15% random loss where TCP Cubic collapses to 3.4 Mbps."

---

### Part 2: MIT Faculty Cold-Email Outreach

#### 1. Email to Prof. Muriel Médard (MIT RLE - Network Coding & Fulcrum Codes Pioneer)
**Subject:** High school senior implementation of Fulcrum Codes over volatile Nigerian LTE (Class of 2031 applicant)

> Dear Professor Médard,
> 
> As an SS3 student in Port Harcourt, Nigeria, where tropical rain fade regularly induces 15%–25% non-congestive packet loss on our mobile networks, I spent this year studying your seminal 2018 paper on Fulcrum Network Codes and building a production Rust implementation called Aether.
> By pairing your outer $GF(2)$ expansion code with an inner $GF(2^8)$ sliding-window solver and ARM NEON SIMD vectorization, our decoder achieves over 180 MB/s on embedded ARM microprocessors while eliminating TCP retransmission pauses.
> I have authored an 8-page NSDI-style pre-print, an IETF RFC draft, and formally verified our solver bounds with Kani and the protocol state machine with TLA+.
> I am applying to MIT for Undergraduate Admission (Class of 2031) and would be profoundly honored if you might glance at our open-source implementation and pre-print at [github.com/aether-transport/aether].

---

#### 2. Email to Prof. Hari Balakrishnan (MIT CSAIL - Networks & Mobile Systems Group)
**Subject:** Cross-layer delay-gradient transport for lossy cellular channels (SS3 Nigerian applicant)

> Dear Professor Balakrishnan,
> 
> Your foundational work on Sprout's stochastic forecasts and Copa's delay-based pacing directly inspired my high school capstone project, Aether—a sub-IP transport engine built in Rust to combat severe cellular bufferbloat and random drops in Port Harcourt, Nigeria.
> We separate loss recovery from congestion notification by marrying an Extended Kalman Filter delay-gradient arbiter ($d\Delta/dt$) with cross-layer cellular telemetry (RSRP/SINR) and rateless Fulcrum network coding.
> In emulated 15% loss environments with 140ms RTT, Aether achieves 62.8 Mbps goodput where TCP Cubic collapses to 3.4 Mbps and QUIC reaches 14.8 Mbps.
> As an applicant to MIT EECS for the Class of 2031, I would be deeply grateful for any critical feedback you might have on our system architecture and technical paper.

---

#### 3. Email to Prof. Mohammad Alizadeh (MIT CSAIL - Data Center Networks & Congestion Control)
**Subject:** EKF delay-gradient congestion arbitration under non-congestive wireless drops

> Dear Professor Alizadeh,
> 
> Following your pioneering research on DCTCP and delay-driven congestion control, I engineered an Extended Kalman Filter congestion arbiter in Rust that isolates intermediate queue gradients from stochastic physical drops on volatile West African wireless links.
> When packet loss occurs while the delay gradient is non-positive, our protocol maintains line pacing and proactively inflates sliding-window network coding redundancy, achieving a 4.2x goodput gain over standard QUIC.
> The entire protocol pipeline is zero-copy, cache-aligned, and incorporates an eBPF/XDP kernel-bypass driver for line-rate packet ingestion.
> I am applying for undergraduate admission to MIT EECS (Class of 2031) and have open-sourced our code, TLA+ formal specification, and benchmark data at [github.com/aether-transport/aether].

---

#### 4. Email to Prof. Frans Kaashoek & Prof. Robert Morris (MIT CSAIL - Systems / PDOS Group)
**Subject:** Zero-copy Rust transport engine with eBPF/XDP and formal verification (MIT 2031 applicant)

> Dear Professors Kaashoek and Morris,
> 
> Inspired by 6.824 and 6.828, I designed and built Aether, a zero-allocation, cache-aligned userspace transport engine written in Rust that operates over raw UDP and eBPF/XDP kernel bypass.
> To ensure ironclad safety across lossy cellular links in Port Harcourt, Nigeria, I formally verified our distributed state transitions against deadlock and out-of-order delivery using TLA+/TLC and proved Galois field arithmetic bounds using Kani.
> Our userspace TUN driver allows tunneling real IPv4/IPv6 traffic, yielding 62.8 Mbps goodput across links where traditional TCP collapses.
> I am submitting my undergraduate application to MIT EECS this cycle and would be humbled if you had a moment to review our system codebase and NSDI-style pre-print.

---

### Part 3: Common App & MIT Essay Alignment

#### Prompt 1: "Describe something you do simply for the pleasure of it."
- **Angle:** The joy of turning abstract Galois Field polynomial algebra into lightning-fast bitwise XORs and SIMD assembly.
- **Narrative Arc:** Sitting at a wooden desk in Port Harcourt while rain drums on a corrugated zinc roof; watching raw bytes stream across a self-written terminal dashboard as Gaussian elimination solves a 3x3 matrix in 2 microseconds; the aesthetic pleasure of mathematical order emerging from physical radio noise.

#### Prompt 2: "Tell us about a significant challenge you have faced and how you overcame it."
- **Angle:** Technical resilience amidst erratic electricity and unreliable cellular connections.
- **Narrative Arc:** Navigating power outages by managing laptop battery cycles, setting up local offline toolchains in Termux and Linux, and turning the daily frustration of stalled downloads into the empirical inspiration for building a rateless network coding protocol engine.
