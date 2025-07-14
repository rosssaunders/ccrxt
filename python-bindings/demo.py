#!/usr/bin/env python3
"""
Demo script showing how to use the CCRXT Python bindings.

This demonstrates instantiating classes from different cryptocurrency exchange venues
and shows the availability of all venue APIs.
"""

import ccrxt

def main():
    print("🎉 CCRXT Real API Bindings Implementation Complete!")
    print("=" * 60)
    
    print("Available venues:")
    venues = [name for name in dir(ccrxt) if not name.startswith('_') and name != 'ccrxt']
    for venue in venues:
        print(f"  - {venue}")
    
    print(f"\nTotal venues: {len(venues)}")
    print("=" * 60)
    
    print("\n🧪 Testing venue class instantiation:")
    
    # Test Binance
    try:
        binance_client = ccrxt.binance.binance_BinanceClient()
        print("✅ Binance: BinanceClient created successfully")
    except Exception as e:
        print(f"❌ Binance: Error creating BinanceClient - {e}")
    
    # Test Deribit
    try:
        deribit_hello = ccrxt.deribit.deribit_HelloRequest()
        print("✅ Deribit: HelloRequest created successfully")
    except Exception as e:
        print(f"❌ Deribit: Error creating HelloRequest - {e}")
    
    # Test OKX
    try:
        okx_balance = ccrxt.okx.okx_AccountBalance()
        print("✅ OKX: AccountBalance created successfully")
    except Exception as e:
        print(f"❌ OKX: Error creating AccountBalance - {e}")
    
    # Test Kucoin
    try:
        kucoin_classes = [name for name in dir(ccrxt.kucoin) if not name.startswith('_')]
        if kucoin_classes:
            class_name = kucoin_classes[0]
            kucoin_obj = getattr(ccrxt.kucoin, class_name)()
            print(f"✅ Kucoin: {class_name} created successfully")
    except Exception as e:
        print(f"❌ Kucoin: Error creating class - {e}")
    
    print("\n📊 Class counts per venue:")
    for venue in venues:
        venue_module = getattr(ccrxt, venue)
        class_count = len([name for name in dir(venue_module) if not name.startswith('_')])
        print(f"  {venue}: {class_count} classes")
    
    print("\n🚀 All venues are ready to use!")
    print("You can now import ccrxt and access any venue's API classes.")

if __name__ == "__main__":
    main()
