# nrs.pub

**Node Resolution Service — Public RPC node resolution and smart routing for any blockchain.**

[Website](https://nrs.pub) | [Chains](https://nrs.pub/chains) | [GitHub](https://github.com/saefstroem/nrs.pub) | [X](https://x.com/saefstroem) | [Donate](https://nrs.pub/donate)

---

## What is NRS?

NRS is a free, open-source service that resolves the best available public RPC endpoint for any supported blockchain. Instead of hardcoding a single RPC URL, point at `nrs.pub/<chain_id>` and get routed to a healthy, vetted public node — every time, a different one.

No API keys. No accounts. No SDK. One URL per chain.

```diff
- const rpc = "https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY";
+ const rpc = "https://nrs.pub/1";
```

Works with anything that follows HTTP redirects: ethers.js, viem, web3.py, curl, wagmi, or any standard JSON-RPC client.

---

## How it works

NRS uses HTTP 307 (Temporary Redirect) to route your request to a healthy public node. It never proxies your traffic — it tells your client where to go, then gets out of the way.

![NRS architecture](arch.png)

1. Your app sends a request to `nrs.pub/<chain_id>`.
2. NRS selects a random healthy node from a curated pool.
3. NRS returns a `307` redirect with the node URL in the `Location` header.
4. Your client follows the redirect and connects directly to the node.
5. The node responds directly to your client. NRS is not involved.

NRS never sees your request payload or the node's response.

---

## Quick start

Replace your existing RPC URL with the NRS endpoint for your chain.

| Chain | Endpoint |
|---|---|
| Ethereum | `https://nrs.pub/1` |
| BNB Smart Chain | `https://nrs.pub/56` |
| Polygon | `https://nrs.pub/137` |
| Arbitrum One | `https://nrs.pub/42161` |
| Base | `https://nrs.pub/8453` |
| Avalanche C-Chain | `https://nrs.pub/43114` |
| Optimism | `https://nrs.pub/10` |

Full list with live status, latency, and 24h uptime: **[nrs.pub/chains](https://nrs.pub/chains)**

<details>
<summary><b>ethers.js</b></summary>

```javascript
import { JsonRpcProvider } from "ethers";
const provider = new JsonRpcProvider("https://nrs.pub/1");
const block = await provider.getBlockNumber();
```
</details>

<details>
<summary><b>viem</b></summary>

```typescript
import { createPublicClient, http } from "viem";
import { mainnet } from "viem/chains";

const client = createPublicClient({
  chain: mainnet,
  transport: http("https://nrs.pub/1"),
});
```
</details>

<details>
<summary><b>web3.py</b></summary>

```python
from web3 import Web3
w3 = Web3(Web3.HTTPProvider("https://nrs.pub/1"))
print(w3.eth.block_number)
```
</details>

<details>
<summary><b>curl</b></summary>

```bash
curl -L -X POST https://nrs.pub/1 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
```

> The `-L` flag (follow redirects) is required for curl. Most libraries follow redirects by default.
</details>

---

## When to use NRS

| Use case | Why NRS fits |
|---|---|
| Prototyping and development | RPC endpoint in 10 seconds, no key registration |
| Multi-chain applications | One URL pattern for 90+ chains instead of 20 provider accounts |
| Scripts, bots, and tooling | Zero-config, no provider account overhead |
| Open-source projects | Ship a working default RPC without requiring users to register for keys |
| Redundancy and failover | Zero-config fallback behind your primary provider |
| Education and hackathons | No signup friction for participants |

### When NRS is not the right choice

- **HFT or MEV** — you need deterministic latency, private mempools, or ordering guarantees.
- **Stateful RPC** — `eth_newFilter` and related methods require consecutive requests to hit the same node.
- **Archive queries** — no guarantee the resolved node is an archive node.
- **Uptime SLAs** — NRS is free infrastructure maintained by one person. No contracts, no guarantees.

---

## For enterprises

NRS is not a replacement for your private RPC infrastructure. It is the fallback layer you should have behind it.

Use your private provider as the primary endpoint and configure NRS as automatic failover:

**viem**
```typescript
import { createPublicClient, http, fallback } from "viem";
import { mainnet } from "viem/chains";

const client = createPublicClient({
  chain: mainnet,
  transport: fallback([
    http("https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY"),
    http("https://nrs.pub/1"),
  ]),
});
```

**ethers.js**
```typescript
import { FallbackProvider, JsonRpcProvider } from "ethers";

const provider = new FallbackProvider([
  new JsonRpcProvider("https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY"),
  new JsonRpcProvider("https://nrs.pub/1"),
]);
```

For critical infrastructure — bridges, oracles, verification networks — issue the same call through both your primary provider and NRS independently, then compare results before acting. If the responses diverge, halt and investigate. This is the cheapest form of cross-provider consensus available today.

### Case study: the KelpDAO hack

On April 18, 2026, attackers linked to North Korea's Lazarus Group stole ~$292 million from KelpDAO's LayerZero bridge. This was not a smart contract exploit — it was an infrastructure attack on the RPC nodes that the verification network depended on.

The verifier relied on a fixed, predictable set of RPC endpoints. The attackers compromised internal nodes and DDoS'd external ones, leaving no honest source of truth. A phantom token burn was fabricated, and the bridge released real funds against it.

If the verification infrastructure had used NRS as a fallback, the DDoS would not have created a total blackout. The system would have resolved to a randomized public node the attacker could not have known about in advance. The forged state would have been contradicted by at least one honest source.

NRS does not guarantee every node in the pool is honest. But it guarantees an attacker cannot predict or control which node will be selected — and that is enough to break the attack model that drained $292 million.

---

## Architecture

NRS is a lightweight Rust service built on [Axum](https://github.com/tokio-rs/axum), routed through multiple load balancers.

- **Axum router** — receives requests at `/:chain_id`, selects a random healthy node, returns a 307 redirect.
- **RPC storage** — in-memory store of chains and endpoints, loaded from a curated JSON configuration.
- **Monitor** — background task that health-checks all nodes via `eth_chainId` with a 3-second timeout. Failed nodes are temporarily excluded.

Node selection uses cryptographically random indexing. If the selected node fails its health check, it's excluded and another is picked. If all nodes for a chain are down, a 500 is returned.

---

## Security model

NRS is an infrastructure primitive, not a security product. It reduces certain classes of risk through randomization and redundancy.

**Mitigates:**
- Single point of failure — if one node goes down, the next request hits another.
- Targeted node compromise — an attacker cannot predict which node serves a given call.
- Provider-level outages — nodes from multiple independent providers per chain.

**Does not mitigate:**
- Malicious node responses — NRS does not validate correctness. For safety-critical operations, use the multi-call pattern: issue the same call 3× through NRS, compare results client-side.
- Privacy — your client connects directly to the node. Use a VPN (see below).
- Consensus — NRS provides resolution, not verification. A `/consensus` endpoint is on the roadmap.

---

## Privacy and data

- **NRS does not store logs or request data.** We return a 307 and nothing else.
- **NRS is not a VPN.** Your client connects directly to the resolved node. The node operator sees your IP and payload.
- **We cannot control what node operators do** with data they receive. Using a VPN with NRS is recommended if tracking is a concern.
- **Node URLs are not published.** Some nodes are shared under confidentiality agreements. We do not disclose individual node infrastructure details.

---

## Acceptable use

- **No node enumeration.** Systematic attempts to discover, map, or fingerprint the node pool are prohibited — including high-volume requests, timing attacks, or response fingerprinting.
- **No abuse.** Using NRS for DDoS amplification, attacks against node operators, or illegal activity is prohibited.

Violations result in automatic IP blacklisting.

---

## Roadmap

- `/consensus` endpoint — proxied, multi-node verified responses 
- Non-EVM chain support — Kaspa, Bitcoin, Solana
- Node reputation scoring — public quality index per node
- WebSocket support — persistent connections with smart routing

---

## Contributing

- **Add a chain** — open a PR with chain ID, name, and reputable public RPC endpoints.
- **Report a bad node** — open an issue if you encounter degraded or incorrect responses.
- **Improve docs** — PRs for documentation and integration guides are welcome.

---

## License

MIT