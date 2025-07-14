#!/usr/bin/env python3
"""
Example showing how to use CCRXT Python bindings with specific venue classes.

This demonstrates creating and using various request/response classes from
different cryptocurrency exchange venues.
"""

import ccrxt

def binance_example():
    """Example using Binance venue classes."""
    print("🔶 Binance Example:")
    
    # Create a Binance client
    client = ccrxt.binance.binance_BinanceClient()
    print("  - BinanceClient created")
    
    # Create various request objects
    ticker_request = ccrxt.binance.binance_TickerRequest()
    print("  - TickerRequest created")
    
    order_request = ccrxt.binance.binance_NewOrderRequest()
    print("  - NewOrderRequest created")
    
    depth_request = ccrxt.binance.binance_DepthRequest()
    print("  - DepthRequest created")
    
    print("  ✅ Binance classes working")

def deribit_example():
    """Example using Deribit venue classes."""
    print("\n🔷 Deribit Example:")
    
    # Create a Deribit client
    client = ccrxt.deribit.deribit_RestClient()
    print("  - RestClient created")
    
    # Create various request objects
    hello_request = ccrxt.deribit.deribit_HelloRequest()
    print("  - HelloRequest created")
    
    order_book_request = ccrxt.deribit.deribit_GetOrderBookRequest()
    print("  - GetOrderBookRequest created")
    
    instruments_request = ccrxt.deribit.deribit_GetInstrumentsRequest()
    print("  - GetInstrumentsRequest created")
    
    print("  ✅ Deribit classes working")

def okx_example():
    """Example using OKX venue classes."""
    print("\n🔸 OKX Example:")
    
    # Create various OKX objects
    account_balance = ccrxt.okx.okx_AccountBalance()
    print("  - AccountBalance created")
    
    account_config = ccrxt.okx.okx_AccountConfig()
    print("  - AccountConfig created")
    
    # Find a request class
    okx_classes = [name for name in dir(ccrxt.okx) if 'Request' in name]
    if okx_classes:
        request_class = getattr(ccrxt.okx, okx_classes[0])
        request_obj = request_class()
        print(f"  - {okx_classes[0]} created")
    
    print("  ✅ OKX classes working")

def kucoin_example():
    """Example using Kucoin venue classes."""
    print("\n🔹 Kucoin Example:")
    
    # Create various Kucoin objects
    kucoin_classes = [name for name in dir(ccrxt.kucoin) if not name.startswith('_')]
    
    # Create a few different types of objects
    for class_name in kucoin_classes[:3]:
        obj = getattr(ccrxt.kucoin, class_name)()
        print(f"  - {class_name} created")
    
    print("  ✅ Kucoin classes working")

def main():
    """Main demonstration function."""
    print("🚀 CCRXT Python Bindings Usage Examples")
    print("=" * 50)
    
    binance_example()
    deribit_example()
    okx_example()
    kucoin_example()
    
    print("\n🎉 All examples completed successfully!")
    print("\n💡 Usage Tips:")
    print("  - Import ccrxt to access all venue modules")
    print("  - Each venue has its own submodule (e.g., ccrxt.binance)")
    print("  - Classes are prefixed with venue name (e.g., binance_BinanceClient)")
    print("  - All classes can be instantiated with default constructors")
    print("  - Use dir(ccrxt.venue) to see available classes for each venue")

if __name__ == "__main__":
    main()
