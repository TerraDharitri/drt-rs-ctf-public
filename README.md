# Capture the flag

Let's see who can get the highest score!

This repo contains 3 challenges:
1. [Bump](ctf-bump/README.md)
2. [Coinflip](ctf-coinflip/README.md)
3. [GasPass](ctf-gaspass/README.md)

All these challenges can be completed using nothing but Rust interactors and smart contracts, but of couse, you are free to use any technology you prefer.

## Setup

Make sure to have your own account, and use a `.pem` file in your interactor, then communicate to the team which one is your wallet.

For convenience, we are also providing an interactor stub. The challenge contracts are already deployed to devnet, you can find their addresses in the interactor stub config:

```
gateway = 'https://devnet-gateway.dharitri.org'
bump_address = "drt1qqqqqqqqqqqqqpgq23j27f6w0r75hfyc5td753f9ahvfpp5x4wzq8g0prs"
coinflip_address = "drt1qqqqqqqqqqqqqpgq0jtfsyk7rfgu50v6wh5wwtk2mjgseca54wzqe0u2fj"
gaspass_address = "drt1qqqqqqqqqqqqqpgq5tlkn9qcza52v4dtfzsx99xjxrgtw2p74wzqwc0h6r"
```

Best of luck!
