#!/usr/bin/env python
import time

from ftxwebsocket.client import FtxWebsocketClient
from rest.client import FtxClient


def ftx_client():
    client = FtxClient(
        "obAqJG1_4_qX4deJ-UwVm6drZ89mafGhd9YkNm8l",
        "7dqybPY84KtZMxL_YAoKMQlQDNuFPAbbF73a8tI5",
        "saber",
    )
    # account = client.get_account_info()
    # print(account)
    # print(client.get_account_info())
    # print(client.get_all_futures())
    # print(client.get_markets())

    ft = FtxWebsocketClient()
    ft.connect()
    # print(ft)
    while True:
        # fills = ft.get_fills()
        # print(fills)
        # time.sleep(1)
        # orders = ft.get_orders()
        # print(orders)
        # time.sleep(1)
        # trades = ft.get_trades("BTC/USD")
        # print(trades)
        # time.sleep(1)
        orderbook = ft.get_orderbook("BTC/USD")
        print(orderbook)
        time.sleep(1)
        pass


def websocket_client():
    import asyncio
    import websockets

    async def hello():
        async with websockets.connect("wss://ftx.us/ws/") as websocket:
            await websocket.send("Hello world!")
            msg = await websocket.recv()
            print(msg)

    asyncio.run(hello())


if __name__ == "__main__":
    # ftx_client()
    websocket_client()
    pass
