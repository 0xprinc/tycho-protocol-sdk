# Tycho Protocol SDK

Tycho Protocol SDK is a library used by solvers to integrate DEX liquidity.

Please refer to the [README.md](docs/README.md) for more information.


# Steps to run the tests
```
aws configure   // shared by tycho team on TG
setup_env.sh
conda activate propeller-protocol-lib-testing
export RPC_URL="https://ethereum-mainnet.core.chainstack.com/123123123123"
export SUBSTREAMS_API_TOKEN=eyJhbGci...             // to be fetched from substreams webpage
docker compose up -d db
python ./testing/src/runner/cli.py --package "ethereum-balancer"
```
