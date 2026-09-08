-- Aether Transport Protocol Wireshark Dissector
-- Author: Aether Project Research Initiative

local aether_proto = Proto("aether", "Aether Transport Protocol")

local f_window_id   = ProtoField.uint32("aether.window_id",   "Window ID",   base.DEC)
local f_symbol_id   = ProtoField.uint16("aether.symbol_id",   "Symbol ID",   base.DEC)
local f_window_size = ProtoField.uint16("aether.window_size", "Window Size", base.DEC)
local f_seed        = ProtoField.uint16("aether.seed",        "PRNG Seed",   base.HEX)
local f_flags       = ProtoField.uint16("aether.flags",       "Flags",       base.HEX)
local f_crc16       = ProtoField.uint16("aether.crc16",       "CRC16",       base.HEX)
local f_auth_tag    = ProtoField.uint16("aether.auth_tag",    "Auth Tag",    base.HEX)
local f_payload     = ProtoField.bytes("aether.payload",      "Symbol Payload")

aether_proto.fields = {
    f_window_id, f_symbol_id, f_window_size, f_seed,
    f_flags, f_crc16, f_auth_tag, f_payload
}

function aether_proto.dissector(buffer, pinfo, tree)
    local length = buffer:len()
    if length < 16 then return end

    pinfo.cols.protocol = "AETHER"

    local subtree = tree:add(aether_proto, buffer(), "Aether Protocol Header")
    
    local win_id = buffer(0, 4):le_uint()
    local sym_id = buffer(4, 2):le_uint()
    local win_sz = buffer(6, 2):le_uint()
    local seed   = buffer(8, 2):le_uint()
    local flags  = buffer(10, 2):le_uint()
    local crc    = buffer(12, 2):le_uint()
    local auth   = buffer(14, 2):le_uint()

    subtree:add_le(f_window_id,   buffer(0, 4))
    subtree:add_le(f_symbol_id,   buffer(4, 2))
    subtree:add_le(f_window_size, buffer(6, 2))
    subtree:add_le(f_seed,        buffer(8, 2))
    subtree:add_le(f_flags,       buffer(10, 2))
    subtree:add_le(f_crc16,       buffer(12, 2))
    subtree:add_le(f_auth_tag,    buffer(14, 2))

    local sym_type = "Systematic"
    if sym_id >= win_sz then
        sym_type = "Fulcrum Repair"
    end

    pinfo.cols.info = string.format("Win: %d, Sym: %d/%d (%s), Seed: 0x%04X", win_id, sym_id, win_sz, sym_type, seed)

    if length > 16 then
        subtree:add(f_payload, buffer(16, length - 16))
    end
end

-- Register on UDP port 4242
local udp_table = DissectorTable.get("udp.port")
udp_table:add(4242, aether_proto)
