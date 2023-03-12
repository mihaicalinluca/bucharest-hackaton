import binascii
import time
import logging
from argparse import ArgumentParser
from erdpy.accounts import Account, Address
from erdpy.contracts import CodeMetadata, SmartContract
from erdpy.interfaces import IElrondProxy
from erdpy.proxy.core import ElrondProxy
from erdpy.transactions import Transaction

# run with python main.py --proxy https://devnet-gateway.multiversx.com --pem devWallet.pem

logger = logging.getLogger("refreshRPS bot")
parser = ArgumentParser()
parser.add_argument("--proxy", required=True)  # put real proxy link in args
parser.add_argument("--pem", required=True)  # put real pem file in args

args = parser.parse_args()

logging.basicConfig(level=logging.DEBUG)

proxy = ElrondProxy(args.proxy)
network = proxy.get_network_config()
user = Account(pem_file=args.pem)

def do_query(sc_address, proxy_url, function, arguments, value, caller):
    contract_address = Address(sc_address)
    contract = SmartContract(address=contract_address)

    answer = contract.query(proxy_url, function, arguments, value, caller)  # arguments is []
    logger.info(f"Answer: {answer}")
    return answer

# fill in the right SC Address
staking_sc_address = "erd1qqqqqqqqqqqqqpgqu6hqeva4sv3gm24e450sgatnpwled2r30yts0ewynn"


# execute tx function, could also use this with the right encoding for the arguments
def do_execute(function, arguments, gas_limit, sc_address):
    contract_address = Address(sc_address)
    contract = SmartContract(address=contract_address)

    user.sync_nonce(proxy)

    tx = contract.execute(
        caller=user,
        function=function,
        arguments=arguments,
        gas_price=network.min_gas_price,
        gas_limit=gas_limit,
        value=0,
        chain=network.chain_id,
        version=network.min_tx_version
    )

    tx_hash = tx.send(proxy)
    logger.info(f"Transaction: {tx_hash}")


def main():
    while True:
        do_execute('computeRps', [], 60000000, staking_sc_address)
        # time.sleep(24 * 60 * 60)   #use this for real time
        time.sleep(60)        # use this for dev

if __name__ == '__main__':
    main()

