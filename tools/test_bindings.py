#!/usr/bin/env python3
"""
Test the PyO3 binding generation with naming conventions
"""

import asyncio
import sys
import os

# Add the python-bindings to path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'python-bindings', 'python'))

async def test_bindings():
    try:
        import ccrxt
        print(f"✅ Successfully imported ccrxt version {ccrxt.__version__}")
        
        # Test basic functionality
        # These will be available once the bindings are generated
        print("📋 Available modules:")
        for attr in dir(ccrxt):
            if not attr.startswith('_'):
                print(f"  - {attr}")
                
    except ImportError as e:
        print(f"❌ Import failed: {e}")
        print("💡 This is expected until you build the bindings with maturin")
        
    except Exception as e:
        print(f"❌ Error: {e}")

async def main():
    print("🔧 Testing CCRXT Python bindings...")
    await test_bindings()
    
    print("\n📝 To build the bindings:")
    print("  cd python-bindings")
    print("  pip install maturin")
    print("  maturin develop")
    print("  cd ..")
    print("  python3 tools/test_bindings.py")

if __name__ == "__main__":
    asyncio.run(main())
