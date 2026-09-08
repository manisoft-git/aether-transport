------------------------- MODULE AetherProtocol -------------------------
EXTENDS Integers, Sequences, FiniteSets, TLC

CONSTANTS 
    K,          \* Number of source symbols per generation / window (e.g. 3)
    MaxRepair,  \* Maximum repair symbols allowed per window (e.g. 2)
    MaxLoss     \* Bound on channel drops for finite model checking

VARIABLES 
    senderState,    \* State of sender: "TRANSMITTING", "AWAITING_ACK", "DONE"
    senderWinId,    \* Current coding window ID at sender
    sentSymbols,    \* Set of symbol IDs transmitted in current window
    channel,        \* Multiset/Set of packets currently in transit
    receiverRank,   \* Current linear rank (degrees of freedom) accumulated at receiver
    receiverWinId,  \* Window ID currently being received
    receiverDecoded,\* Boolean: true if receiver has inverted matrix
    droppedCount    \* Number of packets dropped by the channel

vars == <<senderState, senderWinId, sentSymbols, channel, receiverRank, receiverWinId, receiverDecoded, droppedCount>>

SymType == {"SYSTEMATIC", "FULCRUM_REPAIR"}

Packet == [
    winId : Nat,
    symId : Nat,
    type  : SymType,
    coeffSeed : Nat
]

TypeOK ==
    /\ senderState \in {"TRANSMITTING", "AWAITING_ACK", "DONE"}
    /\ senderWinId \in Nat
    /\ sentSymbols \subseteq Nat
    /\ receiverRank \in 0..K
    /\ receiverWinId \in Nat
    /\ receiverDecoded \in BOOLEAN
    /\ droppedCount \in 0..MaxLoss

Init ==
    /\ senderState = "TRANSMITTING"
    /\ senderWinId = 1
    /\ sentSymbols = {}
    /\ channel = {}
    /\ receiverRank = 0
    /\ receiverWinId = 1
    /\ receiverDecoded = FALSE
    /\ droppedCount = 0

\* Action: Sender transmits systematic symbol
SendSystematic(sId) ==
    /\ senderState = "TRANSMITTING"
    /\ sId \in 1..K
    /\ sId \notin sentSymbols
    /\ sentSymbols' = sentSymbols \cup {sId}
    /\ channel' = channel \cup {[winId |-> senderWinId, symId |-> sId, type |-> "SYSTEMATIC", coeffSeed |-> sId]}
    /\ UNCHANGED <<senderState, senderWinId, receiverRank, receiverWinId, receiverDecoded, droppedCount>>

\* Action: Sender transmits Fulcrum repair symbol
SendRepair(rId) ==
    /\ senderState = "TRANSMITTING"
    /\ Cardinality(sentSymbols) >= K
    /\ rId \in (K + 1)..(K + MaxRepair)
    /\ rId \notin sentSymbols
    /\ sentSymbols' = sentSymbols \cup {rId}
    /\ channel' = channel \cup {[winId |-> senderWinId, symId |-> rId, type |-> "FULCRUM_REPAIR", coeffSeed |-> rId * 31]}
    /\ UNCHANGED <<senderState, senderWinId, receiverRank, receiverWinId, receiverDecoded, droppedCount>>

\* Action: Network channel drops a packet (simulating cellular RF fade)
ChannelDrop(pkt) ==
    /\ pkt \in channel
    /\ droppedCount < MaxLoss
    /\ channel' = channel \ {pkt}
    /\ droppedCount' = droppedCount + 1
    /\ UNCHANGED <<senderState, senderWinId, sentSymbols, receiverRank, receiverWinId, receiverDecoded>>

\* Action: Receiver processes incoming symbol and updates linear rank
ReceiveSymbol(pkt) ==
    /\ pkt \in channel
    /\ channel' = channel \ {pkt}
    /\ IF pkt.winId = receiverWinId /\ ~receiverDecoded
       THEN /\ IF receiverRank + 1 >= K
               THEN /\ receiverRank' = K
                    /\ receiverDecoded' = TRUE
               ELSE /\ receiverRank' = receiverRank + 1
                    /\ UNCHANGED receiverDecoded
       ELSE /\ UNCHANGED <<receiverRank, receiverDecoded>>
    /\ UNCHANGED <<senderState, senderWinId, sentSymbols, receiverWinId, droppedCount>>

\* Action: Receiver acknowledges full decode and sender advances window
AdvanceWindow ==
    /\ receiverDecoded = TRUE
    /\ senderWinId' = senderWinId + 1
    /\ receiverWinId' = receiverWinId + 1
    /\ senderState' = "TRANSMITTING"
    /\ sentSymbols' = {}
    /\ receiverRank' = 0
    /\ receiverDecoded' = FALSE
    /\ UNCHANGED <<channel, droppedCount>>

Next ==
    \/ \E sId \in 1..K: SendSystematic(sId)
    \/ \E rId \in (K + 1)..(K + MaxRepair): SendRepair(rId)
    \/ \E pkt \in channel: ChannelDrop(pkt)
    \/ \E pkt \in channel: ReceiveSymbol(pkt)
    \/ AdvanceWindow

Spec == Init /\ [][Next]_vars /\ WF_vars(Next)

\* -------------------------------------------------------------
\* Formal Invariants
\* -------------------------------------------------------------

\* Safety Invariant: Rank never exceeds K and receiver state is always consistent
Consistency == 
    /\ receiverRank <= K
    /\ (receiverDecoded => receiverRank = K)
    /\ (receiverWinId <= senderWinId)

\* Liveness Invariant: If K linearly independent packets arrive, reconstruction succeeds
ReconstructionProgress ==
    (receiverRank = K) ~> receiverDecoded

=============================================================================
