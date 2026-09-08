// SPDX-License-Identifier: (GPL-2.0 OR BSD-2-Clause)
// eBPF/XDP Kernel-Bypass Filter for Aether Transport Protocol

#include <linux/bpf.h>
#include <linux/if_ether.h>
#include <linux/ip.h>
#include <linux/udp.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_endian.h>

#define AETHER_UDP_PORT 4242

struct {
    __uint(type, BPF_MAP_TYPE_XSKMAP);
    __uint(max_entries, 64);
    __uint(key_size, sizeof(int));
    __uint(value_size, sizeof(int));
} xsks_map SEC(".maps");

SEC("xdp")
int xdp_aether_filter(struct xdp_md *ctx) {
    void *data_end = (void *)(long)ctx->data_end;
    void *data = (void *)(long)ctx->data;

    struct ethhdr *eth = data;
    if ((void *)(eth + 1) > data_end)
        return XDP_PASS;

    if (eth->h_proto != bpf_htons(ETH_P_IP))
        return XDP_PASS;

    struct iphdr *ip = (void *)(eth + 1);
    if ((void *)(ip + 1) > data_end)
        return XDP_PASS;

    if (ip->protocol != IPPROTO_UDP)
        return XDP_PASS;

    struct udphdr *udp = (void *)(ip + 1);
    if ((void *)(udp + 1) > data_end)
        return XDP_PASS;

    // Check if UDP destination is Aether port 4242
    if (bpf_ntohs(udp->dest) == AETHER_UDP_PORT) {
        // Redirect directly into AF_XDP socket UMEM ring buffer
        int index = ctx->rx_queue_index;
        return bpf_redirect_map(&xsks_map, index, 0);
    }

    return XDP_PASS;
}

char _license[] SEC("license") = "Dual BSD/GPL";
